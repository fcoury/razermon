use razermacos::RazerDevices;
use rusqlite::OptionalExtension;
use std::fmt;

use crate::database;

#[derive(Clone, Debug, serde::Serialize)]
pub(crate) struct BatteryStatus {
    pub device_id: u32,
    pub name: String,
    pub percentage: u8,
    pub charging: bool,
}

impl BatteryStatus {
    pub fn get(device_id: u32) -> Option<Self> {
        let mut devices = RazerDevices::new();
        let device = devices.find(device_id as u16);

        if let Some(device) = device {
            return Some(BatteryStatus {
                device_id,
                name: device.name.clone(),
                percentage: device.battery(),
                charging: device.is_charging(),
            });
        }

        None
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let db = database::Conn::new()?;
        let charging = if self.charging { 1 } else { 0 };
        db.conn.execute(
            "INSERT INTO battery (device_id, percentage, charging) VALUES (?1, ?2, ?3)",
            (&self.device_id, &self.percentage, &charging),
        )?;
        Ok(())
    }

    pub fn last_status() -> anyhow::Result<Option<u8>> {
        let db = database::Conn::new()?;
        let percentage: Option<u8> = db.conn.query_row(
            "SELECT percentage FROM battery WHERE percentage > 0 ORDER BY created_at DESC LIMIT 1",
            [],
            |row| row.get::<usize, u8>(0),
        ).optional()?;
        Ok(percentage)
    }
}

impl fmt::Display for BatteryStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let icon = if self.charging {
            "⚡️"
        } else if self.percentage > 60 {
            "🔋"
        } else if self.percentage > 20 {
            "🪫"
        } else {
            "🔌"
        };
        write!(f, "{}{}%", icon, self.percentage)
    }
}
