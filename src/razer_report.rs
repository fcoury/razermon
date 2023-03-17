use std::{thread, time::Duration};

use associated::Associated;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use hidapi::HidDevice;
use strum::{Display, FromRepr};

use crate::RazerError;

/// Different status codes that can be sent in the razer report
#[repr(u8)]
#[derive(FromRepr, Display, Debug, Clone, Copy)]
pub enum RazerStatus {
    /// Used to send commands to the device
    NewCommand = 0,
    /// The device is too busy to respond
    CommandBusy = 1,
    /// The command returned correctly
    CommandSuccessful = 2,
    /// The command failed to run
    CommandFailure = 3,
    /// The command timed out
    CommandNoResponseOrTimeout = 4,
    /// The command is not supported by this device
    CommandNotSupport = 5,
}

/// Some magic that some devices need to be different. What each value means is still unsure.
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum RazerTransactionDevice {
    Default = 0xE0,
    Zero = 0x00,
    One = 0x20,
    Four = 0x80,
}

/// Some magic that all devices send the same magic for.
#[repr(u8)]
#[derive(Clone, Copy)]
pub(crate) enum RazerTransactionId {
    Default = 0x1F,
}

/// Different groups of commands that can be sent to the keyboard
/// get_razer_report(0x03, 0x00, 0x03) -> 0x03 is the class
#[repr(u8)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub(crate) enum RazerCommandClass {
    StandardDevice = 0x00,
    StandardLED = 0x03,
    ExtendedMatrix = 0x0F,
    ExtendedMatrixMouse = 0x0D,
    Misc = 0x07,
    Blade = 0x0E,
}

/// The direction of the command being sent.
#[repr(u8)]
#[derive(Clone, Copy)]
pub(crate) enum RazerCommandDirection {
    /// Used to set settings on the device
    HostToDevice = 0x00,
    /// Used to get settings on the device
    DeviceToHost = 0x80,
}

/// Special magic for any given command
#[derive(Clone, Copy)]
pub(crate) struct RazerCommandParts(RazerCommandClass, u8, u8);

/// Every kind of command that be sent in a command
#[derive(Associated)]
#[associated(Type = RazerCommandParts)]
#[allow(dead_code)]
pub(crate) enum RazerCommand {
    #[assoc_const(RazerCommandParts(RazerCommandClass::StandardDevice, 0x04, 2))]
    DeviceMode,
    #[assoc_const(RazerCommandParts(RazerCommandClass::StandardDevice, 0x02, 22))]
    Serial,
    #[assoc_const(RazerCommandParts(RazerCommandClass::StandardDevice, 0x01, 2))]
    FirmwareVersion,
    #[assoc_const(RazerCommandParts(RazerCommandClass::StandardLED, 0x00, 3))]
    LedState,
    #[assoc_const(RazerCommandParts(RazerCommandClass::StandardLED, 0x04, 4))]
    LedBlinking,
    #[assoc_const(RazerCommandParts(RazerCommandClass::StandardLED, 0x01, 5))]
    LedRgb,
    #[assoc_const(RazerCommandParts(RazerCommandClass::StandardLED, 0x02, 3))]
    LedEffect,
    #[assoc_const(RazerCommandParts(RazerCommandClass::StandardLED, 0x03, 3))]
    LedBrightness,
    #[assoc_const(RazerCommandParts(RazerCommandClass::StandardLED, 0x0A, 8))]
    StandardMatrixEffect,
    #[assoc_const(RazerCommandParts(RazerCommandClass::StandardLED, 0x0B, 70))]
    StandardMatrixCustomFrame,
    #[assoc_const(RazerCommandParts(RazerCommandClass::ExtendedMatrix, 0x04, 3))]
    ExtendedMatrixBrightness,
    #[assoc_const(RazerCommandParts(RazerCommandClass::Blade, 0x04, 2))]
    BladeBrightness,
    #[assoc_const(RazerCommandParts(RazerCommandClass::Misc, 0x80, 2))]
    BatteryCharge,
    #[assoc_const(RazerCommandParts(RazerCommandClass::Misc, 0x84, 2))]
    ChargingStatus,
}

pub(crate) struct RazerReport {
    report_id: u8,
    status: RazerStatus,
    transaction_device: RazerTransactionDevice,
    remaining_packets: u16, /* Big Endian */
    command_direction: RazerCommandDirection,
    command: RazerCommand,
    body: Bytes,
}

impl RazerReport {
    pub fn new(
        command_direction: RazerCommandDirection,
        command: RazerCommand,
        body: Bytes,
        transaction_device: RazerTransactionDevice,
    ) -> Self {
        let report_id = 0u8;

        RazerReport {
            report_id,
            status: RazerStatus::NewCommand,
            transaction_device,
            remaining_packets: 0, //this seems to always be 0, so 🤷‍♀️
            command_direction,
            command,
            body,
        }
    }
    fn create_payload(&self) -> Bytes {
        let mut buf = BytesMut::with_capacity(90);
        buf.put_u8(self.report_id);
        buf.put_u8(self.status as u8);
        buf.put_u8(self.transaction_device as u8 | RazerTransactionId::Default as u8); //transaction_id
        buf.put_u16(self.remaining_packets); // crate will magically convert to big endian
        buf.put_u8(0u8); //protocol_type

        let RazerCommandParts(command_class, command_id, read_data_size) =
            self.command.get_associated();
        let data_size: u8 = match self.command_direction {
            RazerCommandDirection::DeviceToHost => *read_data_size,
            RazerCommandDirection::HostToDevice => self.body.len().try_into().unwrap(),
        };
        buf.put_u8(data_size); //data_size
        buf.put_u8(*command_class as u8);
        buf.put_u8(command_id | self.command_direction as u8);
        buf.put(self.body.clone());
        let mut crc = 0u8;
        for byte in buf.clone().into_iter().skip(3) {
            crc ^= byte;
        }
        buf.put_bytes(0, 80 - self.body.len()); // zero out remaining body
        buf.put_u8(crc);
        buf.put_u8(0u8); //reserved
        buf.freeze()
    }

    fn verify_response(&self, response_buffer: &[u8]) -> Result<Bytes, RazerError> {
        let mut response = Bytes::copy_from_slice(response_buffer);
        if response.get_u8() != self.report_id {
            return Err(RazerError::MismatchResponse("report id"));
        }
        let status = RazerStatus::from_repr(response.get_u8());
        match status {
            None => return Err(RazerError::FailedToParse("invalid status".to_string())),
            Some(real_status) => match real_status {
                RazerStatus::CommandSuccessful => (),
                _ => return Err(RazerError::BadStatus(real_status)),
            },
        };
        response.advance(1);
        if response.get_u16() != self.remaining_packets {
            return Err(RazerError::MismatchResponse("remaining packets"));
        }
        response.advance(1);
        let response_data_size = response.get_u8();
        if response_data_size > 80 {
            return Err(RazerError::FailedToParse("invalid data size".to_string()));
        }

        let RazerCommandParts(command_class, command_id, read_data_size) =
            self.command.get_associated();
        if response_data_size != *read_data_size {
            return Err(RazerError::MismatchResponse("wrong size packet"));
        }
        if response.get_u8() != *command_class as u8 {
            return Err(RazerError::MismatchResponse("command class"));
        }
        if response.get_u8() != command_id | self.command_direction as u8 {
            return Err(RazerError::MismatchResponse("command id"));
        }
        Ok(response.copy_to_bytes(response_data_size.into()))
    }

    pub fn send_packet(&self, hid_device: &HidDevice) -> Result<(), RazerError> {
        let sent_packet = self.create_payload();
        hid_device.send_feature_report(sent_packet.as_ref())?;
        Ok(())
    }

    pub fn receive_packet(&self, hid_device: &HidDevice) -> Result<Bytes, RazerError> {
        let mut buf = [0u8; 91];
        buf[0] = self.report_id;
        hid_device.get_feature_report(&mut buf)?;
        self.verify_response(&buf)
    }

    pub fn send_and_receive_packet(&self, hid_device: &HidDevice) -> Result<Bytes, RazerError> {
        self.send_packet(hid_device)?;
        // thread::yield_now();
        thread::sleep(Duration::from_micros(59900));
        self.receive_packet(hid_device)
    }
}

/// Every kind of LED on razer devices
#[repr(u8)]
#[derive(FromRepr, Display, Debug, Clone, Copy)]
pub enum RazerLed {
    Zero = 0x00,
    ScrollWheel = 0x01,
    Battery = 0x03,
    Logo = 0x04,
    Backlight = 0x05,
    Macro = 0x07,
    Game = 0x08,
    RedProfile = 0x0C,
    GreenProfile = 0x0D,
    BlueProfile = 0x0E,
    RightSide = 0x10,
    LeftSide = 0x11,
    Charging = 0x20,
    FastCharging = 0x21,
    FullyCharging = 0x22,
}

/// Tells the device if it should remember the command, or just set it temporarily.
///
/// Not all devices support saving, and also saving too often can wear down memory.
#[repr(u8)]
#[derive(FromRepr, Display, Debug, Clone, Copy)]
pub enum RazerStorage {
    /// Only set value temporarily
    NoStore = 0x00,
    /// Save setting to survive restart
    VarStore = 0x01,
}
