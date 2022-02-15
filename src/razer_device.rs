use crate::razer_report::*;
use bytes::{Buf, Bytes};
use hidapi::HidDevice;
use std::fmt::Display;
use strum::{Display, FromRepr};

pub const RAZER_VENDOR_ID: u16 = 0x1532;

#[derive(Clone, Copy)]
pub struct RazerDeviceConnectInfo {
    pub interface_number: Option<i32>,
    pub usage: Option<u16>,
    pub usage_page: Option<u16>,
}

pub trait RazerDeviceKind {
    fn get_transaction_device(&self) -> RazerTransactionDevice;
}

pub struct RazerDevice<T>
where
    T: RazerDeviceKind,
{
    pub kind: T,
    pub name: String,
    pub(crate) serial: Option<String>,
    pub(crate) hid_device: HidDevice,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RazerFirmwareVersion(u8, u8);

impl Display for RazerFirmwareVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v{}.{}", self.0, self.1)
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Display, FromRepr, Debug, PartialEq)]
pub enum DeviceMode {
    Normal = 0x00,
    FactoryTesting = 0x02,
    Driver = 0x03,
}

impl<T> RazerDevice<T>
where
    T: RazerDeviceKind,
{
    pub fn new(kind: T, name: String, serial: Option<String>, hid_device: HidDevice) -> Self {
        RazerDevice {
            kind,
            name,
            serial,
            hid_device,
        }
    }

    pub fn get_firmware_version(&self) -> Result<RazerFirmwareVersion, RazerReportError> {
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

    pub fn get_serial(&self) -> Result<String, RazerReportError> {
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

    pub fn get_device_mode(&self) -> Result<DeviceMode, RazerReportError> {
        let report = RazerReport::new(
            RazerCommandDirection::DeviceToHost,
            RazerCommand::DeviceMode,
            Bytes::new(),
            self.kind.get_transaction_device(),
        );
        let mut response_payload = report.send_and_receive_packet(&self.hid_device)?;
        DeviceMode::from_repr(response_payload.get_u8())
            .ok_or_else(|| RazerReportError::FailedToParse("invalid device mode".into()))
    }

    pub fn set_device_mode(&self, mode: DeviceMode) -> Result<(), RazerReportError> {
        let report = RazerReport::new(
            RazerCommandDirection::HostToDevice,
            RazerCommand::DeviceMode,
            vec![mode as u8, 0u8].into(),
            self.kind.get_transaction_device(),
        );
        report.send_packet(&self.hid_device)
    }

    pub fn set_led_brightness(
        &self,
        store: RazerStorage,
        led: RazerLed,
        percent: u8,
    ) -> Result<(), RazerReportError> {
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

    pub fn get_led_brightness(
        &self,
        store: RazerStorage,
        led: RazerLed,
    ) -> Result<u8, RazerReportError> {
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

    pub fn set_extended_matrix_brightness(
        &self,
        store: RazerStorage,
        led: RazerLed,
        percent: u8,
    ) -> Result<(), RazerReportError> {
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

    pub fn get_extended_matrix_brightness(
        &self,
        store: RazerStorage,
        led: RazerLed,
    ) -> Result<u8, RazerReportError> {
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
}
