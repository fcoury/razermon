use crate::{
    razer_keyboard::RazerKeyboardKind,
    razer_report::{RazerReport, RazerReportError},
};
use bytes::Bytes;
use hidapi::HidDevice;
use std::fmt::Display;

pub const RAZER_VENDOR_ID: u16 = 0x1532;

#[derive(Clone, Copy)]
pub struct RazerDeviceConnectInfo {
    pub interface_number: Option<i32>,
    pub usage: Option<u16>,
    pub usage_page: Option<u16>,
}

pub trait RazerDeviceKind {}

pub struct RazerDevice<T>
where
    T: RazerDeviceKind,
{
    kind: T,
    hid_device: HidDevice,
}
pub struct RazerFirmwareVersion(u8, u8);

impl Display for RazerFirmwareVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v{}.{}", self.0, self.1)
    }
}

impl<T> RazerDevice<T>
where
    T: RazerDeviceKind,
{
    pub fn new(kind: T, hid_device: HidDevice) -> Self {
        RazerDevice { kind, hid_device }
    }

    pub fn get_firmware_version(&self) -> Result<RazerFirmwareVersion, RazerReportError> {
        let report = RazerReport::new(
            crate::razer_report::RazerCommandDirection::DeviceToHost,
            crate::razer_report::RazerCommand::FirmwareVersion,
            Bytes::new(),
            None,
        );
        let response_payload = report.send_and_receive_packet(&self.hid_device)?;
        if response_payload.len() != 2 {
            return Err(RazerReportError::FailedToParse);
        }
        Ok(RazerFirmwareVersion(
            *response_payload.get(0).unwrap(),
            *response_payload.get(1).unwrap(),
        ))
    }
}
