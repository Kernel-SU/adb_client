#![crate_type = "lib"]
#![forbid(unsafe_code)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_docs)]
#![doc = include_str!("../README.md")]

mod adb_device_ext;
mod adb_transport;
mod error;
mod message_devices;
mod models;
mod utils;

pub use adb_device_ext::ADBDeviceExt;
pub use error::{Result, RustADBError};
pub use message_devices::*;
