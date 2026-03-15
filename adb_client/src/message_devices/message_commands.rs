use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt::Display;

use crate::message_devices::utils::BinaryEncodable;

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MessageCommand {
    /// Connect to a device
    Cnxn = 0x4E58_4E43,
    /// Close connection to a device
    Clse = 0x4553_4C43,
    /// Device ask for authentication
    Auth = 0x4854_5541,
    /// Open a data connection
    Open = 0x4E45_504F,
    /// Write data to connection
    Write = 0x4554_5257,
    /// Server understood the message
    Okay = 0x5941_4B4F,
}

impl BinaryEncodable for MessageCommand {
    fn encode(&self) -> Vec<u8> {
        u32::from(*self).to_le_bytes().to_vec()
    }
}

impl Display for MessageCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cnxn => write!(f, "CNXN"),
            Self::Clse => write!(f, "CLSE"),
            Self::Auth => write!(f, "AUTH"),
            Self::Open => write!(f, "OPEN"),
            Self::Write => write!(f, "WRTE"),
            Self::Okay => write!(f, "OKAY"),
        }
    }
}
