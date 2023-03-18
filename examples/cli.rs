use std::env;

use razer_driver_rs::{
    razer_report::{RazerLed, RazerStorage},
    scan_for_devices,
};

fn main() {
    let command = env::args().nth(1).expect("Usage: battery <command>");
    let devices = scan_for_devices().unwrap();
    let mut devices = devices.into_iter().collect::<Vec<_>>();
    devices.sort_by(|a, b| a.name.cmp(&b.name));

    match command.as_ref() {
        "list" => {
            println!("Found {} device(s):", &devices.len());
            for (i, device) in devices.iter().enumerate() {
                println!("  {}. {} ({:?})", i + 1, device.name, device.kind);
            }
        }
        "battery" => {
            for device in devices {
                let bat = device.get_battery_charge();
                let charging = device.get_charging_status();

                match (bat, charging) {
                    (Ok(bat), Ok(charging)) => {
                        let percentage = bat as f32 / 255.0 * 100.0;
                        let charging = if charging == 1 { "⚡️" } else { "🔋" };
                        println!("{} => {charging}{percentage:.0}%", device.name,);
                    }
                    _ => {
                        println!("{} => No info", device.name);
                    }
                }
            }
        }
        "ledoff" => {
            let index = env::args().nth(2).expect("Usage: battery ledoff <index>");
            let index: usize = index.parse().expect("Invalid index");
            let device = devices.get(index - 1).expect("Invalid index");
            println!("Turning off LED on {}", device.name);
            device
                .set_led_brightness(RazerStorage::NoStore, RazerLed::Zero, 0)
                .unwrap();
        }
        "led" => {
            let index = env::args().nth(2).expect("Usage: battery led <index>");
            let index: usize = index.parse().expect("Invalid index");
            let device = devices.get(index - 1).expect("Invalid index");
            let value = device
                .get_led_state(RazerStorage::VarStore, RazerLed::Logo)
                .unwrap();
            println!("Led value for {} = {}", device.name, value);
        }
        _ => {}
    };
}
