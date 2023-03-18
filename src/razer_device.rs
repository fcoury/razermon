use crate::{
    razer_keyboard::RazerKeyboardKind, razer_mouse::RazerMouseKind, razer_report::*, RazerError,
};
use bytes::{Buf, Bytes};
use hidapi::HidDevice;
use std::fmt::Display;
use strum::{Display, FromRepr};

/// All Razer devices have this vendor ID
pub const RAZER_VENDOR_ID: u16 = 0x1532;

/// Infomation for connecting to the management device of the the razer device
#[derive(Clone, Copy, Debug)]
pub struct RazerDeviceConnectInfo {
    pub interface_number: Option<i32>,
    pub usage: Option<u16>,
    pub usage_page: Option<u16>,
}

/// Represents different types of devices (eg. keyboards vs mice vs headsets)
pub trait RazerDeviceKind {
    /// Returns the specific transaction device that this device needs
    fn get_transaction_device(&self) -> RazerTransactionDevice;
}

/// Represents the connection to the device
#[derive(Debug)]
pub struct RazerDevice<T>
where
    T: RazerDeviceKind,
{
    pub kind: T,
    pub name: String,
    pub(crate) serial: Option<String>,
    pub(crate) hid_device: HidDevice,
}

#[derive(Debug)]
pub enum RazerDeviceType {
    Keyboard(RazerKeyboardKind),
    Mouse(RazerMouseKind),
}

impl RazerDeviceKind for RazerDeviceType {
    fn get_transaction_device(&self) -> RazerTransactionDevice {
        match self {
            RazerDeviceType::Keyboard(kind) => kind.get_transaction_device(),
            RazerDeviceType::Mouse(kind) => kind.get_transaction_device(),
        }
    }
}

/// The major and minor version of the firmware of the device
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RazerFirmwareVersion(u8, u8);

impl Display for RazerFirmwareVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v{}.{}", self.0, self.1)
    }
}

/// The modes the device can be set for
#[repr(u8)]
#[derive(Clone, Copy, Display, FromRepr, Debug, PartialEq)]
pub enum DeviceMode {
    /// The unmanaged mode the keyboard will be in without any special software
    Normal = 0x00,
    /// Some kind of factory testing mode that causes the FN and macro keys to behave like standard keys
    FactoryTesting = 0x02,
    /// The keyboard will behave like there is the full razer software installed
    Driver = 0x03,
}

impl<T> RazerDevice<T>
where
    T: RazerDeviceKind,
{
    /// Reads the firmware version of the device
    pub fn get_firmware_version(&self) -> Result<RazerFirmwareVersion, RazerError> {
        let report = RazerReport::new(
            RazerCommandDirection::DeviceToHost,
            RazerCommand::FirmwareVersion,
            Bytes::new(),
            self.kind.get_transaction_device(),
        );
        let mut response_payload = report.send_and_receive_packet(&self.hid_device)?;
        Ok(RazerFirmwareVersion(
            response_payload.get_u8(),
            response_payload.get_u8(),
        ))
    }

    /// Reads the serial number of the device
    pub fn get_serial(&self) -> Result<String, RazerError> {
        if let Some(serial) = &self.serial {
            if !serial.is_empty() {
                return Ok(serial.to_owned());
            }
        }
        let report = RazerReport::new(
            RazerCommandDirection::DeviceToHost,
            RazerCommand::Serial,
            Bytes::new(),
            self.kind.get_transaction_device(),
        );
        report
            .send_and_receive_packet(&self.hid_device)
            .map(|x| String::from_utf8_lossy(x.as_ref()).to_string())
    }

    /// Reads the device mode
    pub fn get_device_mode(&self) -> Result<DeviceMode, RazerError> {
        let report = RazerReport::new(
            RazerCommandDirection::DeviceToHost,
            RazerCommand::DeviceMode,
            Bytes::new(),
            self.kind.get_transaction_device(),
        );
        let mut response_payload = report.send_and_receive_packet(&self.hid_device)?;
        DeviceMode::from_repr(response_payload.get_u8())
            .ok_or_else(|| RazerError::FailedToParse("invalid device mode".into()))
    }

    /// Sets the device mode
    pub fn set_device_mode(&self, mode: DeviceMode) -> Result<(), RazerError> {
        let report = RazerReport::new(
            RazerCommandDirection::HostToDevice,
            RazerCommand::DeviceMode,
            vec![mode as u8, 0u8].into(),
            self.kind.get_transaction_device(),
        );
        report.send_packet(&self.hid_device)
    }

    /// Sets the LED brightness of the device.
    /// You must pass through how the keyboard saves the result and which LED is being set.
    pub fn set_led_brightness(
        &self,
        store: RazerStorage,
        led: RazerLed,
        percent: u8,
    ) -> Result<(), RazerError> {
        if percent > 100 {
            panic!("cannot set brightness to more than 100");
        }
        let report = RazerReport::new(
            RazerCommandDirection::HostToDevice,
            RazerCommand::LedBrightness,
            vec![store as u8, led as u8, percent].into(),
            self.kind.get_transaction_device(),
        );
        report.send_packet(&self.hid_device)
    }

    /// Gets the LED brightness of the device.
    /// You must pass through how the keyboard saved the result and which LED is being read from.
    pub fn get_led_brightness(&self, store: RazerStorage, led: RazerLed) -> Result<u8, RazerError> {
        let report = RazerReport::new(
            RazerCommandDirection::DeviceToHost,
            RazerCommand::LedBrightness,
            vec![store as u8, led as u8].into(),
            self.kind.get_transaction_device(),
        );
        let mut response = report.send_and_receive_packet(&self.hid_device)?;
        response.advance(2);
        Ok(response.get_u8())
    }

    /// Gets the Battery charge percentage.
    pub fn get_battery_charge(&self) -> Result<u8, RazerError> {
        let report = RazerReport::new(
            RazerCommandDirection::DeviceToHost,
            RazerCommand::BatteryCharge,
            vec![0x00, 0x00].into(),
            self.kind.get_transaction_device(),
        );
        let mut response = report.send_and_receive_packet(&self.hid_device)?;
        response.advance(1);
        Ok(response.get_u8())
    }

    /// Gets the Battery charge percentage.
    pub fn get_charging_status(&self) -> Result<u8, RazerError> {
        let report = RazerReport::new(
            RazerCommandDirection::DeviceToHost,
            RazerCommand::ChargingStatus,
            vec![0x00, 0x00].into(),
            self.kind.get_transaction_device(),
        );
        let mut response = report.send_and_receive_packet(&self.hid_device)?;
        response.advance(1);
        Ok(response.get_u8())
    }

    /// Sets the matric brightness of the device.
    /// You must pass through how the keyboard saves the result and which LED matrix is being set.
    pub fn set_extended_matrix_brightness(
        &self,
        store: RazerStorage,
        led: RazerLed,
        percent: u8,
    ) -> Result<(), RazerError> {
        if percent > 100 {
            panic!("cannot set brightness to more than 100");
        }
        let report = RazerReport::new(
            RazerCommandDirection::HostToDevice,
            RazerCommand::ExtendedMatrixBrightness,
            vec![store as u8, led as u8, percent].into(),
            self.kind.get_transaction_device(),
        );
        report.send_packet(&self.hid_device)?;
        Ok(())
    }

    /// Gets the matric brightness of the device.
    /// You must pass through how the keyboard saved the result and which LED matrix is being read.
    pub fn get_extended_matrix_brightness(
        &self,
        store: RazerStorage,
        led: RazerLed,
    ) -> Result<u8, RazerError> {
        let report = RazerReport::new(
            RazerCommandDirection::DeviceToHost,
            RazerCommand::ExtendedMatrixBrightness,
            vec![store as u8, led as u8].into(),
            self.kind.get_transaction_device(),
        );
        let mut response = report.send_and_receive_packet(&self.hid_device)?;
        response.advance(2);
        Ok(response.get_u8())
    }

    pub fn get_led_state(&self, store: RazerStorage, led: RazerLed) -> Result<u8, RazerError> {
        let report = RazerReport::new(
            RazerCommandDirection::DeviceToHost,
            RazerCommand::LedState,
            vec![store as u8, led as u8].into(),
            self.kind.get_transaction_device(),
        );
        let mut response = report.send_and_receive_packet(&self.hid_device)?;
        response.advance(2);
        Ok(response.get_u8())
    }
}
