use crate::razer_device::{RazerDevice, RazerDeviceConnectInfo, RazerDeviceKind};
use associated::Associated;
use strum::FromRepr;

#[derive(Associated, FromRepr, Debug, PartialEq, Clone, Copy)]
#[repr(u16)]
#[associated(Type = RazerDeviceConnectInfo)]
pub enum RazerKeyboardKind {
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlackwidowUltimate2012 = 0x010d,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlackwidowStealthEdition = 0x010e,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerAnansi = 0x010f,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerNostromo = 0x0111,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerOrbweaver = 0x0113,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlackwidowUltimate2013 = 0x011a,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlackwidowStealth = 0x011b,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlackwidowTe2014 = 0x011c,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerTartarus = 0x0201,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerDeathstalkerExpert = 0x0202,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlackwidowChroma = 0x0203,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerDeathstalkerChroma = 0x0204,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBladeStealth = 0x0205,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerOrbweaverChroma = 0x0207,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerTartarusChroma = 0x0208,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlackwidowChromaTe = 0x0209,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBladeQhd = 0x020f,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBladeProLate2016 = 0x0210,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlackwidowOverwatch = 0x0211,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlackwidowUltimate2016 = 0x0214,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlackwidowXChroma = 0x0216,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlackwidowXUltimate = 0x0217,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlackwidowXChromaTe = 0x021a,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerOrnataChroma = 0x021e,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerOrnata = 0x021f,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBladeStealthLate2016 = 0x0220,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlackwidowChromaV2 = 0x0221,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBladeLate2016 = 0x0224,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBladePro2017 = 0x0225,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerHuntsmanElite = 0x0226,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerHuntsman = 0x0227,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlackwidowElite = 0x0228,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerCynosaChroma = 0x022a,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerTartarusV2 = 0x022b,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerCynosaChromaPro = 0x022c,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBladeStealthMid2017 = 0x022d,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBladePro2017Fullhd = 0x022f,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBladeStealthLate2017 = 0x0232,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlade2018 = 0x0233,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBladePro2019 = 0x0234,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlackwidowLite = 0x0235,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlackwidowEssential = 0x0237,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBladeStealth2019 = 0x0239,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlade2019Adv = 0x023a,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlade2018Base = 0x023b,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerCynosaLite = 0x023f,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlade2018Mercury = 0x0240,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlackwidow2019 = 0x0241,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerHuntsmanTe = 0x0243,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBladeMid2019Mercury = 0x0245,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlade2019Base = 0x0246,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBladeStealthLate2019 = 0x024a,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBladeProLate2019 = 0x024c,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBladeStudioEdition2019 = 0x024d,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(3), usage: Some(1), usage_page: Some(0x0C)})]
    RazerBlackwidowV3 = 0x024e,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBladeStealthEarly2020 = 0x0252,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlade15Adv2020 = 0x0253,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBladeEarly2020Base = 0x0255,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBladeProEarly2020 = 0x0256,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerHuntsmanMini = 0x0257,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(3), usage: Some(1), usage_page: Some(0x0C)})]
    RazerBlackwidowV3Mini = 0x0258,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBladeStealthLate2020 = 0x0259,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlackwidowV3ProWired = 0x025a,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(3), usage: Some(1), usage_page: Some(0x0C)})]
    RazerBlackwidowV3ProWireless = 0x025c,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerOrnataV2 = 0x025d,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerCynosaV2 = 0x025e,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(3), usage: Some(1), usage_page: Some(0x0C)})]
    RazerHuntsmanV2Analog = 0x0266,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerHuntsmanMiniJp = 0x0269,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBook2020 = 0x026a,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(3), usage: Some(1), usage_page: Some(0x0C)})]
    RazerHuntsmanV2TKL = 0x026b,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(3), usage: Some(1), usage_page: Some(0x0C)})]
    RazerHuntsmanV2 = 0x026c,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlade15AdvEarly2021 = 0x026d,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlade15BaseEarly2021 = 0x026f,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlade142021 = 0x0270,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(3), usage: Some(1), usage_page: Some(0x0C)})]
    RazerBlackwidowV3MiniWireless = 0x0271,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlade15AdvMid2021 = 0x0276,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlade17ProMid2021 = 0x0279,
    #[assoc_const(RazerDeviceConnectInfo {interface_number: Some(2), usage: Some(2), usage_page: Some(1)})]
    RazerBlackwidowV3Tk = 0x0a24,
}

impl RazerKeyboardKind {
    pub fn is_blade(&self) -> bool {
        match self {
            &RazerKeyboardKind::RazerBladeStealth
            | &RazerKeyboardKind::RazerBladeStealthLate2016 => true,
            _ => false,
        }
    }
}

impl RazerDeviceKind for RazerKeyboardKind {}

impl RazerDevice<RazerKeyboardKind> {}
