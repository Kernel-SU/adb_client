use std::io::Write;
use std::net::SocketAddr;

use crate::message_devices::adb_message_device::ADBMessageDevice;
use crate::tcp::tcp_transport::TcpTransport;
use crate::{ADBDeviceExt, Result};

/// Represent a device reached and available over TCP.
#[derive(Debug)]
pub struct ADBTcpDevice {
    inner: ADBMessageDevice<TcpTransport>,
}

impl ADBTcpDevice {
    /// Instantiate a new [`ADBTcpDevice`]
    pub fn new<A: Into<SocketAddr>>(address: A) -> Result<Self> {
        Ok(Self {
            inner: ADBMessageDevice::new(TcpTransport::new(address))?,
        })
    }
}

impl ADBDeviceExt for ADBTcpDevice {
    #[inline]
    fn shell_command(
        &mut self,
        command: &dyn AsRef<str>,
        stdout: Option<&mut dyn Write>,
        stderr: Option<&mut dyn Write>,
    ) -> Result<Option<u8>> {
        self.inner.shell_command(command, stdout, stderr)
    }

    #[inline]
    fn root(&mut self) -> Result<()> {
        self.inner.root()
    }
}
