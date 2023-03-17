use razer_driver_rs::scan_for_devices;

fn main() {
    for device in scan_for_devices().unwrap().into_iter() {
        println!("Name: {}", device.name);
        let bat = device.get_battery_charge().unwrap();
        let percentage = bat as f32 / 255.0 * 100.0;
        let charging = device.get_charging_status().unwrap();
        println!("Battery: {:.0}%", percentage);
        println!("Charging: {}", charging);
    }
}
