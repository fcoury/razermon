use razermacos::RazerDevices;
use std::fmt;

#[derive(Clone, Debug, serde::Serialize)]
pub(crate) struct BatteryStatus {
    pub name: String,
    pub percentage: u8,
    pub charging: bool,
}

impl BatteryStatus {
    pub fn get(device_id: u32) -> Option<Self> {
        let mut devices = RazerDevices::init();
        let device = devices.find(device_id as u16);

        if let Some(device) = device {
            return Some(BatteryStatus {
                name: device.name.clone(),
                percentage: device.battery(),
                charging: device.is_charging(),
            });
        }

        None
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
