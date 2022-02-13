extern crate hidapi;
pub mod razer_device;
pub mod razer_report;

use associated::Associated;
use hidapi::HidApi;
use razer_device::{RazerDevice, RazerDeviceKind};
use std::error::Error;

pub fn scan_for_devices() -> Result<Vec<RazerDevice>, Box<dyn Error>> {
    let mut devices = Vec::new();
    let api = HidApi::new()?;
    for device in api.device_list() {
        if device.vendor_id() == razer_device::RAZER_VENDOR_ID {
            if let Some(valid_device) =
                razer_device::RazerKeyboardKind::from_repr(device.product_id())
            {
                let connect_info = valid_device.get_associated();
                if (connect_info.interface_number == None
                    || connect_info.interface_number == Some(device.interface_number()))
                    && (connect_info.usage == None || connect_info.usage == Some(device.usage()))
                    && (connect_info.usage_page == None
                        || connect_info.usage_page == Some(device.usage_page()))
                {
                    println!(
                        "found {:?}, {:#04x}, {}, {}, {}, {}",
                        device.serial_number(),
                        device.product_id(),
                        device.interface_number(),
                        device.usage(),
                        device.usage_page(),
                        device.path().to_string_lossy(),
                    );
                    if let Ok(hid_device) = device.open_device(&api) {
                        devices.push(RazerDevice::new(
                            RazerDeviceKind::Keyboard(valid_device),
                            hid_device,
                        ));
                    }
                }
            }
        }
    }
    Ok(devices)
}

#[cfg(test)]
mod tests {
    use crate::scan_for_devices;

    #[test]
    fn it_works() {
        let devices = scan_for_devices().unwrap();
        assert_eq!(devices.len(), 1);
        let keyboard = devices.get(0).unwrap();
        let version = keyboard.get_firmware_version().unwrap();
        println!("{}", version);
        assert_eq!("v2.1", version.to_string());
    }
    // #[test]
    // fn direct_connect() {
    //     let keyboard = connect_to_device(crate::RazerDeviceKind::Keyboard(
    //         crate::razer_device::RazerKeyboardKind::RazerBlackwidowStealth,
    //     ))
    //     .unwrap();
    //     assert_eq!("v2.1", keyboard.get_firmware_version().unwrap());
    // }
}
