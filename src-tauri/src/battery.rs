use razermacos::RazerDevices;
use rusqlite::OptionalExtension;
use std::fmt;

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
        let mut devices = RazerDevices::new();
        let device = devices.find(product_id);

        if let Some(device) = device {
            return Some(BatteryStatus {
                product_id,
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
            "INSERT INTO battery (product_id, percentage, charging) VALUES (?1, ?2, ?3)",
            (&self.product_id, &self.percentage, &charging),
        )?;
        Ok(())
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
}

impl fmt::Display for BatteryStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let icon = if self.charging {
            "⚡️"
        } else if self.percentage > 20 {
            "🔋"
        } else if self.percentage > 10 {
            "🪫"
        } else {
            "🔌"
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
}
