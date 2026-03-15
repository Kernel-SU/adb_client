use std::io::Write;

use crate::Result;

/// Trait representing all features available on ADB devices.
pub trait ADBDeviceExt {
    /// Runs command in a shell on the device, and write its output and error streams into output.
    fn shell_command(
        &mut self,
        command: &dyn AsRef<str>,
        stdout: Option<&mut dyn Write>,
        stderr: Option<&mut dyn Write>,
    ) -> Result<Option<u8>>;

    /// Restart adb daemon with root permissions
    fn root(&mut self) -> Result<()>;

    /// Return a boxed instance representing this trait
    fn boxed(self) -> Box<dyn ADBDeviceExt>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}
