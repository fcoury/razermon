use std::error::Error;

use associated::Associated;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use hidapi::{HidDevice, HidError};
use strum::{Display, FromRepr};
#[repr(u8)]
#[derive(FromRepr, Debug, Clone, Copy)]
pub enum RazerStatus {
    NewCommand = 0,
    CommandBusy = 1,
    CommandSuccessful = 2,
    CommandFailure = 3,
    CommandNoResponseOrTimeout = 4,
    CommandNotSupport = 5,
}
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum RazerTransactionDevice {
    Default = 0xE0,
    Zero = 0x00,
    One = 0x20,
    Four = 0x80,
}
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum RazerTransactionId {
    Default = 0x1F,
}
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum RazerCommandClass {
    StandardDevice = 0x00,
    StandardLED = 0x03,
    ExtendedMatrix = 0x0F,
    ExtendedMatrixMouse = 0x0D,
    Misc = 0x07,
}
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum RazerCommandDirection {
    HostToDevice = 0x00,
    DeviceToHost = 0x80,
}

#[derive(Clone, Copy)]
pub struct RazerCommandParts(RazerCommandClass, u8, u8);

#[derive(Associated)]
#[associated(Type = RazerCommandParts)]
pub enum RazerCommand {
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
}

#[derive(Display, Debug)]
pub enum RazerReportError {
    MismatchResponse(&'static str),
    BadStatus(RazerStatus),
    FailedToParse,
    HidError(HidError),
}

impl Error for RazerReportError {}

// impl fmt::Display for RazerReportError<'_> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "razer error: {}", self.0)
//     }
// }

pub struct RazerReport {
    report_id: u8,
    status: RazerStatus,
    transaction_device: RazerTransactionDevice,
    remaining_packets: u16, /* Big Endian */
    command_direction: RazerCommandDirection,
    command: RazerCommand,
    body: Bytes,
}

impl RazerReport {
    pub fn new<TOptionalTransaction>(
        command_direction: RazerCommandDirection,
        command: RazerCommand,
        body: Bytes,
        optional_transaction_device: TOptionalTransaction,
    ) -> Self
    where
        TOptionalTransaction: Into<Option<RazerTransactionDevice>>,
    {
        let report_id = 0u8;
        let transaction_device = optional_transaction_device
            .into()
            .unwrap_or(RazerTransactionDevice::Default);

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

    fn verify_response(&self, response_buffer: &[u8]) -> Result<Bytes, RazerReportError> {
        let mut response = Bytes::copy_from_slice(response_buffer);
        if response.get_u8() != self.report_id {
            return Err(RazerReportError::MismatchResponse("report id"));
        }
        let status = RazerStatus::from_repr(response.get_u8());
        match status {
            None => return Err(RazerReportError::FailedToParse),
            Some(real_status) => match real_status {
                RazerStatus::CommandSuccessful => (),
                _ => return Err(RazerReportError::BadStatus(real_status)),
            },
        };
        response.advance(1);
        if response.get_u16() != self.remaining_packets {
            return Err(RazerReportError::MismatchResponse("remaining packets"));
        }
        response.advance(1);
        let response_data_size = response.get_u8();
        if response_data_size > 80 {
            return Err(RazerReportError::FailedToParse);
        }
        let RazerCommandParts(command_class, command_id, _read_data_size) =
            self.command.get_associated();
        if response.get_u8() != *command_class as u8 {
            return Err(RazerReportError::MismatchResponse("command class"));
        }
        if response.get_u8() != command_id | self.command_direction as u8 {
            return Err(RazerReportError::MismatchResponse("command id"));
        }
        Ok(response.copy_to_bytes(response_data_size.into()))
    }

    pub fn send_packet(&self, hid_device: &HidDevice) -> Result<(), RazerReportError> {
        let sent_packet = self.create_payload();
        hid_device
            .send_feature_report(sent_packet.as_ref())
            .map_err(RazerReportError::HidError)
    }

    pub fn receive_packet(&self, hid_device: &HidDevice) -> Result<Bytes, RazerReportError> {
        let mut buf = [0u8; 91];
        buf[0] = self.report_id;
        hid_device
            .get_feature_report(&mut buf)
            .map_err(RazerReportError::HidError)?;
        self.verify_response(&buf)
    }

    pub fn send_and_receive_packet(
        &self,
        hid_device: &HidDevice,
    ) -> Result<Bytes, RazerReportError> {
        self.send_packet(hid_device)?;
        self.receive_packet(hid_device)
    }
}
