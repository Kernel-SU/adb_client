use std::io::Write;

use crate::models::ADBLocalCommand;
use crate::{
    Result,
    message_devices::{
        adb_message_device::ADBMessageDevice, adb_message_transport::ADBMessageTransport,
        message_commands::MessageCommand,
    },
};

impl<T: ADBMessageTransport> ADBMessageDevice<T> {
    /// Runs 'command' in a shell on the device, and write its output and error streams into output.
    pub(crate) fn shell_command(
        &mut self,
        command: &dyn AsRef<str>,
        mut stdout: Option<&mut dyn Write>,
        _stderr: Option<&mut dyn Write>,
    ) -> Result<Option<u8>> {
        let mut session = self.open_session(&ADBLocalCommand::ShellCommand(
            command.as_ref().to_string(),
            Vec::new(),
        ))?;

        loop {
            let message = session.recv_and_reply_okay()?;
            if message.header().command() == MessageCommand::Clse {
                break;
            }
            if let Some(ref mut stdout) = stdout {
                stdout.write_all(&message.into_payload())?;
            }
        }

        Ok(None)
    }
}
