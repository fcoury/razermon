use crate::{
    razer_device::{RazerDeviceConnectInfo, RazerDeviceKind},
    razer_report::*,
};
use associated::Associated;
use strum::{Display, FromRepr};

/// Every kind of keyboard that this library can talk to.
/// Has an associated type that has information on how to connect to the device.
#[derive(Associated, FromRepr, Debug, Display, PartialEq, Clone, Copy)]
#[repr(u16)]
#[associated(Type = RazerDeviceConnectInfo)]
pub enum RazerMouseKind {
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(0), usage: Some(2), usage_page: Some(1)})]
    MouseDockPro = 0x00a4,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(0), usage: Some(2), usage_page: Some(1)})]
    BasiliskV3ProWireless = 0x00ab,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(0), usage: Some(2), usage_page: Some(1)})]
    BasiliskV3ProWirelessDongle = 0x00aa,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(0), usage: Some(2), usage_page: Some(1)})]
    ViperUltimateWireless = 0x007b,
}

impl RazerDeviceKind for RazerMouseKind {
    fn get_transaction_device(&self) -> RazerTransactionDevice {
        match self {
            RazerMouseKind::MouseDockPro => RazerTransactionDevice::Default,
            RazerMouseKind::BasiliskV3ProWireless => RazerTransactionDevice::Default,
            RazerMouseKind::BasiliskV3ProWirelessDongle => RazerTransactionDevice::Default,
            RazerMouseKind::ViperUltimateWireless => RazerTransactionDevice::Default,
        }
    }
}
