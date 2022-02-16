//! Provides an interface for interacting with Razer devices.
//!
//! Abstracts the USB HID interface and the data structure of the communication protocol.
//!
//! ## Example
//! A basic example would be
//! ```rust
//! use razer_driver_rs::*;
//!
//! let devices = scan_for_devices()?;
//! let keyboard = devices.keyboards.get(0).unwrap();
//! let brightness = keyboard.get_brightness()?;
//! println!("brightness {}", brightness);
//! keyboard.set_brightness(90)?;
//! ```

extern crate hidapi;
pub mod razer_device;
pub mod razer_keyboard;
pub mod razer_report;

use associated::Associated;
use hidapi::{HidApi, HidError};
use razer_device::RazerDevice;
use razer_keyboard::RazerKeyboardKind;
use razer_report::RazerStatus;
use thiserror::Error;

/// Structure that contains every type of device found connected to the computer
#[derive(Default)]
pub struct FoundRazerDevices {
    pub keyboards: Vec<RazerDevice<RazerKeyboardKind>>,
}

/// Entry point for interacting with any device. Finds anything connected to the computer.
///
/// The way the HIDAPI library works means we cannot cheaply connect to an individual device,
/// so even if you want something specific this function would be just as fast.
pub fn scan_for_devices() -> Result<FoundRazerDevices, RazerError> {
    let mut devices = FoundRazerDevices::default();
    let api = HidApi::new()?;
    for device in api.device_list() {
        if device.vendor_id() == razer_device::RAZER_VENDOR_ID {
            if let Some(valid_device) = RazerKeyboardKind::from_repr(device.product_id()) {
                let connect_info = valid_device.get_associated();
                if (connect_info.interface_number == None
                    || connect_info.interface_number == Some(device.interface_number()))
                    && (connect_info.usage == None || connect_info.usage == Some(device.usage()))
                    && (connect_info.usage_page == None
                        || connect_info.usage_page == Some(device.usage_page()))
                {
                    let name = match device.product_string() {
                        Some(x) => x.to_string(),
                        None => valid_device.to_string(),
                    };
                    let serial = device.serial_number().map(|x| x.to_string());
                    if let Ok(hid_device) = device.open_device(&api) {
                        devices.keyboards.push(RazerDevice::new(
                            valid_device,
                            name,
                            serial,
                            hid_device,
                        ));
                    }
                }
            }
        }
    }
    Ok(devices)
}

/// Every kind of error that might be sent from this library
#[derive(Error, Debug)]
pub enum RazerError {
    #[error("response data did not have expected value in field {0}. Likely a response to the wrong request")]
    MismatchResponse(&'static str),
    #[error("response from hardware is an error {0}")]
    BadStatus(RazerStatus),
    #[error("response contained invalid data layout {0}")]
    FailedToParse(String),
    #[error("an error occured at the HID level")]
    HidError(#[from] HidError),
}

/// Result type for razer commands
pub type RazerResult<T> = Result<T, RazerError>;

#[cfg(test)]
mod tests {
    use crate::scan_for_devices;

    #[test]
    fn it_works() {
        let devices = scan_for_devices().unwrap();
        assert_eq!(devices.keyboards.len(), 1);
        let keyboard = devices.keyboards.get(0).unwrap();
        let version = keyboard.get_firmware_version().unwrap();
        println!("{}", version);
        assert_eq!("v2.1", version.to_string());
        println!("brightness {}", keyboard.get_brightness().unwrap());

        keyboard.set_brightness(90).unwrap();
        let brightness = keyboard.get_brightness().unwrap();
        println!("brightness {}", brightness);
        assert_eq!(90, brightness);
        println!("serial {}", keyboard.get_serial().unwrap())
    }
    #[test]
    fn test_setting_factory_mode() {
        let devices = scan_for_devices().unwrap();
        assert_eq!(devices.keyboards.len(), 1);
        let keyboard = devices.keyboards.get(0).unwrap();
        keyboard
            .set_device_mode(crate::razer_device::DeviceMode::FactoryTesting)
            .unwrap();
        let mode = keyboard.get_device_mode().unwrap();
        assert_eq!(crate::razer_device::DeviceMode::FactoryTesting, mode);
    }
}
