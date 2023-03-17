use std::fmt;

use crate::human_display::HumanDuration;
use chrono::{Duration, NaiveDateTime};
use razer_driver_rs::scan_mice;
use razermacos::RazerDevices;
use rusqlite::OptionalExtension;

use crate::database;

#[derive(Clone, Debug, serde::Serialize)]
pub(crate) struct BatteryStatus {
    pub product_id: u16,
    pub name: String,
    pub percentage: u8,
    pub charging: bool,
}

impl BatteryStatus {
    pub fn get(product_id: u16) -> Option<Self> {
        let device = scan_mice(product_id);
        let Ok(device) = device else {
            return None;
        };

        if let Some(device) = device {
            let battery = device.get_battery_charge().unwrap();
            let percentage = (battery as f32 / 255.0 * 100.0).round() as u8;
            let charging = device.get_charging_status().unwrap() == 1;

            return Some(BatteryStatus {
                product_id,
                name: device.name.clone(),
                percentage,
                charging,
            });
        }

        None
    }

    pub fn last_status(product_id: u16) -> anyhow::Result<Option<u8>> {
        let db = database::Conn::new()?;
        let percentage: Option<u8> = db.conn.query_row(
            "SELECT percentage FROM battery WHERE percentage > 0 AND product_id = ?1 ORDER BY created_at DESC LIMIT 1",
            [&product_id],
            |row| row.get::<usize, u8>(0),
        ).optional()?;
        Ok(percentage)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let db = database::Conn::new()?;
        let charging = if self.charging { 1 } else { 0 };
        db.conn.execute(
            "INSERT INTO battery (product_id, percentage, charging) VALUES (?1, ?2, ?3)",
            (&self.product_id, &self.percentage, &charging),
        )?;
        Ok(())
    }

    pub fn remaining(&self) -> anyhow::Result<Option<Duration>> {
        let entries = BatteryData::get(self.product_id)?;
        let consumption = BatteryData::consumption(&entries);
        match consumption {
            Some(consumption) => Ok(Some(Duration::seconds(
                consumption * self.percentage as i64,
            ))),
            None => Ok(None),
        }
    }

    pub fn fmt_remaining(&self) -> anyhow::Result<Option<String>> {
        let duration = self.remaining()?;
        match duration {
            Some(duration) => Ok(Some(duration.as_human().to_string())),
            None => Ok(None),
        }
    }
}

impl fmt::Display for BatteryStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let icon = if self.charging {
            "⚡️"
        } else if self.percentage > 20 {
            "🔋"
        } else if self.percentage > 10 {
            "🪫"
        } else if self.percentage > 0 {
            "🔌"
        } else {
            "💤"
        };
        write!(f, "{}{}%", icon, self.percentage)
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub(crate) struct BatteryData {
    pub product_id: u16,
    pub created_at: String,
    pub percentage: u8,
    pub charging: bool,
}

impl BatteryData {
    #[allow(dead_code)]
    pub fn new(product_id: u16, created_at: &str, percentage: u8, charging: bool) -> Self {
        Self {
            product_id,
            created_at: created_at.to_string(),
            percentage,
            charging,
        }
    }

    pub fn get(product_id: u16) -> anyhow::Result<Vec<BatteryData>> {
        let db = database::Conn::new()?.conn;
        let mut statement = db
            .prepare("SELECT * FROM battery WHERE product_id = ?1")
            .unwrap();
        let res = serde_rusqlite::from_rows::<BatteryData>(statement.query([&product_id]).unwrap());
        let res = res.collect::<Vec<_>>();
        let res = res.into_iter().collect::<Result<Vec<_>, _>>();
        Ok(res?)
    }

    pub fn timestamp(&self) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(&self.created_at, "%Y-%m-%d %H:%M:%S").unwrap()
    }

    /// Calculates the average time it takes for the battery to lose 1% of charge
    ///
    /// - Calculates the time it takes to discharge each 1% of the battery (in seconds)
    /// - If the current charge percentage is zero, it means that the device is sleeping, so
    /// accumulate the time between the last charge and the current time
    /// - The next time the charge percentage is greater than zero, it means that the device
    /// woke up, so calculate keep the start date as the last time the percentage was greater than
    /// zero
    /// - When the charge percentage drop 1% again, calculate the first time for the last
    /// percentage, taking out the accumulated idle time
    pub fn consumption(entries: &Vec<BatteryData>) -> Option<i64> {
        let mut measurements = vec![];
        let mut idle_intervals = vec![];
        let mut last_entry: Option<&BatteryData> = None;
        let mut last_line_entry: Option<&BatteryData> = None;
        for entry in entries {
            if entry.percentage > 0 {
                if let Some(cur_last_entry) = last_entry {
                    if entry.percentage != cur_last_entry.percentage {
                        // gets the duration between current and last entries, discounting the current idle time
                        // println!(
                        //     "\nCalculating duration between {:?} ({}) and {:?} ({})",
                        //     entry.percentage,
                        //     entry.timestamp(),
                        //     cur_last_entry.percentage,
                        //     cur_last_entry.timestamp(),
                        // );
                        let duration = entry.timestamp() - cur_last_entry.timestamp();
                        // println!("  - raw duration: {:?}", duration);
                        let idle_time_seconds = idle_intervals.iter().sum::<i64>();
                        // println!("  - idle time: {:?}", idle_time_seconds);
                        let idle_time = chrono::Duration::seconds(idle_time_seconds as i64);
                        // println!("  - idle time (duration): {:?}", idle_time);
                        let duration = duration - idle_time;
                        // println!("  - duration: {:?}", duration);
                        measurements.push(duration);

                        last_entry = Some(entry);
                        idle_intervals = vec![];
                    }
                } else {
                    last_entry = Some(entry);
                    idle_intervals = vec![];
                }
            } else {
                if let Some(last_entry) = last_line_entry {
                    // gets the duration between current and last entries
                    let idle_duration = entry.timestamp() - last_entry.timestamp();
                    // println!(
                    //     "  [!] adding idle time between {} ({:?}) and {} ({:?}) = {:?}",
                    //     entry.percentage,
                    //     entry.timestamp(),
                    //     last_entry.percentage,
                    //     last_entry.timestamp(),
                    //     idle_duration
                    // );
                    idle_intervals.push(idle_duration.num_seconds() as i64);
                }
            }
            last_line_entry = Some(entry);
        }
        let total_time = measurements.iter().map(|d| d.num_seconds()).sum::<i64>();
        // TODO: Make this an Option and return None when there are no measurements to assure we're
        // telling the user it's to early to have an idea on how much the battery will last
        if measurements.len() < 1 {
            return None;
        }
        Some(total_time / measurements.len() as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battery_duration() {
        let entries = vec![
            BatteryData::new(1, "2022-01-01 20:49:40", 76, false),
            BatteryData::new(1, "2022-01-01 21:09:49", 75, false),
            BatteryData::new(1, "2022-01-01 21:51:08", 74, false),
            BatteryData::new(1, "2022-01-01 22:27:49", 0, false),
            BatteryData::new(1, "2022-01-01 23:19:10", 74, false),
            BatteryData::new(1, "2022-01-01 23:27:59", 0, false),
            BatteryData::new(1, "2022-01-01 23:45:20", 74, false),
            BatteryData::new(1, "2022-01-01 23:51:35", 73, false),
        ];

        let duration = BatteryData::consumption(&entries);
        assert_eq!(duration, Some(2728));
    }

    #[test]
    fn test_no_measurements() {
        let duration = BatteryData::consumption(&vec![]);
        assert_eq!(duration, None);
    }
}
