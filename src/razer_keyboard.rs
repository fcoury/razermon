use crate::{
    razer_device::{RazerDevice, RazerDeviceConnectInfo, RazerDeviceKind},
    razer_report::*,
    RazerError,
};
use associated::Associated;
use bytes::Buf;
use strum::{Display, FromRepr};
use RazerKeyboardKind::*;

/// Every kind of keyboard that this library can talk to.
/// Has an associated type that has information on how to connect to the device.
#[derive(Associated, FromRepr, Debug, Display, PartialEq, Clone, Copy)]
#[repr(u16)]
#[associated(Type = RazerDeviceConnectInfo)]
pub enum RazerKeyboardKind {
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BlackwidowUltimate2012 = 0x010d,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BlackwidowStealthEdition = 0x010e,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    Anansi = 0x010f,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    Nostromo = 0x0111,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    Orbweaver = 0x0113,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BlackwidowUltimate2013 = 0x011a,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BlackwidowStealth = 0x011b,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BlackwidowTe2014 = 0x011c,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    Tartarus = 0x0201,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    DeathstalkerExpert = 0x0202,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BlackwidowChroma = 0x0203,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    DeathstalkerChroma = 0x0204,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BladeStealth = 0x0205,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    OrbweaverChroma = 0x0207,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    TartarusChroma = 0x0208,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BlackwidowChromaTe = 0x0209,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BladeQhd = 0x020f,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BladeProLate2016 = 0x0210,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BlackwidowOverwatch = 0x0211,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BlackwidowUltimate2016 = 0x0214,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BlackwidowXChroma = 0x0216,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BlackwidowXUltimate = 0x0217,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BlackwidowXChromaTe = 0x021a,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    OrnataChroma = 0x021e,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    Ornata = 0x021f,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BladeStealthLate2016 = 0x0220,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BlackwidowChromaV2 = 0x0221,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BladeLate2016 = 0x0224,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BladePro2017 = 0x0225,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    HuntsmanElite = 0x0226,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    Huntsman = 0x0227,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BlackwidowElite = 0x0228,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    CynosaChroma = 0x022a,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    TartarusV2 = 0x022b,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    CynosaChromaPro = 0x022c,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BladeStealthMid2017 = 0x022d,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BladePro2017Fullhd = 0x022f,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BladeStealthLate2017 = 0x0232,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    Blade2018 = 0x0233,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BladePro2019 = 0x0234,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BlackwidowLite = 0x0235,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BlackwidowEssential = 0x0237,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BladeStealth2019 = 0x0239,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    Blade2019Adv = 0x023a,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    Blade2018Base = 0x023b,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    CynosaLite = 0x023f,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    Blade2018Mercury = 0x0240,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    Blackwidow2019 = 0x0241,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    HuntsmanTe = 0x0243,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BladeMid2019Mercury = 0x0245,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    Blade2019Base = 0x0246,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BladeStealthLate2019 = 0x024a,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BladeProLate2019 = 0x024c,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BladeStudioEdition2019 = 0x024d,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(3), usage: Some(1), usage_page: Some(0x0C)})]
    BlackwidowV3 = 0x024e,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BladeStealthEarly2020 = 0x0252,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    Blade15Adv2020 = 0x0253,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BladeEarly2020Base = 0x0255,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BladeProEarly2020 = 0x0256,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    HuntsmanMini = 0x0257,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(3), usage: Some(1), usage_page: Some(0x0C)})]
    BlackwidowV3Mini = 0x0258,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BladeStealthLate2020 = 0x0259,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BlackwidowV3ProWired = 0x025a,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(3), usage: Some(1), usage_page: Some(0x0C)})]
    BlackwidowV3ProWireless = 0x025c,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    OrnataV2 = 0x025d,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    CynosaV2 = 0x025e,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(3), usage: Some(1), usage_page: Some(0x0C)})]
    HuntsmanV2Analog = 0x0266,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    HuntsmanMiniJp = 0x0269,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    Book2020 = 0x026a,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(3), usage: Some(1), usage_page: Some(0x0C)})]
    HuntsmanV2TKL = 0x026b,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(3), usage: Some(1), usage_page: Some(0x0C)})]
    HuntsmanV2 = 0x026c,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    Blade15AdvEarly2021 = 0x026d,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    Blade15BaseEarly2021 = 0x026f,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    Blade142021 = 0x0270,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(3), usage: Some(1), usage_page: Some(0x0C)})]
    BlackwidowV3MiniWireless = 0x0271,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    Blade15AdvMid2021 = 0x0276,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    Blade17ProMid2021 = 0x0279,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    BlackwidowV3Tk = 0x0a24,
}

impl RazerKeyboardKind {
    /// Returns true if the device a blade laptop
    pub fn is_blade(&self) -> bool {
        matches!(
            self,
            BladeStealth
                | BladeStealthLate2016
                | BladeProLate2016
                | Blade2018
                | Blade2018Mercury
                | Blade2018Base
                | Blade2019Adv
                | BladeMid2019Mercury
                | BladeStudioEdition2019
                | BladeQhd
                | BladeLate2016
                | BladeStealthMid2017
                | BladeStealthLate2017
                | BladeStealth2019
                | BladePro2017
                | BladePro2017Fullhd
                | Blade2019Base
                | BladeStealthLate2019
                | BladePro2019
                | BladeProLate2019
                | BladeStealthEarly2020
                | BladeStealthLate2020
                | BladeProEarly2020
                | Book2020
                | Blade15Adv2020
                | BladeEarly2020Base
                | Blade15AdvEarly2021
                | Blade15AdvMid2021
                | Blade15BaseEarly2021
                | Blade17ProMid2021
                | Blade142021
        )
    }
    /// Returns true is the keyboard only has a logo LED for programing
    pub fn is_logo_only(&self) -> bool {
        matches!(
            self,
            BlackwidowStealth
                | BlackwidowStealthEdition
                | BlackwidowUltimate2012
                | BlackwidowUltimate2013
                | BlackwidowTe2014
        )
    }
    /// Returns true if keyboard supports extended matrix LEDs
    pub fn is_extended_matrix(&self) -> bool {
        matches!(
            self,
            TartarusV2
                | Ornata
                | OrnataChroma
                | HuntsmanElite
                | HuntsmanTe
                | HuntsmanMini
                | HuntsmanMiniJp
                | Blackwidow2019
                | Huntsman
                | BlackwidowEssential
                | CynosaChroma
                | CynosaChromaPro
                | CynosaLite
                | BlackwidowV3
                | BlackwidowV3Tk
                | BlackwidowV3ProWired
                | BlackwidowElite
                | CynosaV2
                | OrnataV2
                | HuntsmanV2Analog
                | BlackwidowV3Mini
                | BlackwidowV3MiniWireless
        )
    }
}

impl RazerDeviceKind for RazerKeyboardKind {
    fn get_transaction_device(&self) -> RazerTransactionDevice {
        match self {
            TartarusV2 | BlackwidowElite | CynosaV2 | OrnataV2 | HuntsmanV2Analog
            | BlackwidowV3Mini => RazerTransactionDevice::Zero,
            BlackwidowV3MiniWireless => RazerTransactionDevice::Four,
            _ => RazerTransactionDevice::Default,
        }
    }
}

impl RazerDevice<RazerKeyboardKind> {
    /// Sets the brightness of the keyboard.
    /// Will use a different method, depending on what features the keyboard has.
    pub fn set_brightness(&self, percent: u8) -> Result<(), RazerError> {
        match self.kind {
            TartarusV2 => {
                self.set_extended_matrix_brightness(RazerStorage::VarStore, RazerLed::Zero, percent)
            }
            k if k.is_logo_only() => {
                self.set_led_brightness(RazerStorage::VarStore, RazerLed::Logo, percent)
            }
            k if k.is_extended_matrix() => self.set_extended_matrix_brightness(
                RazerStorage::VarStore,
                RazerLed::Backlight,
                percent,
            ),
            k if k.is_blade() => self.set_blade_brightness(percent),
            _ => self.set_led_brightness(RazerStorage::VarStore, RazerLed::Backlight, percent),
        }
    }
    /// Gets the brightness of the keyboard.
    /// Will use a different method, depending on what features the keyboard has.
    pub fn get_brightness(&self) -> Result<u8, RazerError> {
        match self.kind {
            TartarusV2 => {
                self.get_extended_matrix_brightness(RazerStorage::VarStore, RazerLed::Zero)
            }
            k if k.is_logo_only() => {
                self.get_led_brightness(RazerStorage::VarStore, RazerLed::Logo)
            }
            k if k.is_extended_matrix() => {
                self.get_extended_matrix_brightness(RazerStorage::VarStore, RazerLed::Backlight)
            }
            k if k.is_blade() => self.get_blade_brightness(),
            _ => self.get_led_brightness(RazerStorage::VarStore, RazerLed::Backlight),
        }
    }
    /// Sets the brightness of the blade laptop keyboard
    pub fn set_blade_brightness(&self, percent: u8) -> Result<(), RazerError> {
        if percent > 100 {
            panic!("cannot set brightness to more than 100");
        }
        let report = RazerReport::new(
            RazerCommandDirection::HostToDevice,
            RazerCommand::BladeBrightness,
            vec![1u8, percent].into(),
            self.kind.get_transaction_device(),
        );
        report.send_packet(&self.hid_device)
    }
    /// Gets the brightness of the blade laptop keyboard
    pub fn get_blade_brightness(&self) -> Result<u8, RazerError> {
        let report = RazerReport::new(
            RazerCommandDirection::DeviceToHost,
            RazerCommand::BladeBrightness,
            vec![1u8].into(),
            self.kind.get_transaction_device(),
        );
        let mut response = report.send_and_receive_packet(&self.hid_device)?;
        response.advance(1);
        Ok(response.get_u8())
    }
}
