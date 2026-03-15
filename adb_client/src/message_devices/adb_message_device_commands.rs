use crate::{
    ADBDeviceExt, Result,
    message_devices::{
        adb_message_device::ADBMessageDevice, adb_message_transport::ADBMessageTransport,
    },
};
use std::io::Write;

impl<T: ADBMessageTransport> ADBDeviceExt for ADBMessageDevice<T> {
    #[inline]
    fn shell_command(
        &mut self,
        command: &dyn AsRef<str>,
        stdout: Option<&mut dyn Write>,
        stderr: Option<&mut dyn Write>,
    ) -> Result<Option<u8>> {
        self.shell_command(command, stdout, stderr)
    }

    #[inline]
    fn root(&mut self) -> Result<()> {
        self.root()
    }
}
