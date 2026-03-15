use rand::RngExt;

use crate::{
    Result, RustADBError,
    message_devices::{
        adb_message_transport::ADBMessageTransport, adb_session::ADBSession,
        adb_transport_message::ADBTransportMessage, message_commands::MessageCommand,
    },
    models::ADBLocalCommand,
};

/// Generic structure representing an ADB device reachable over an [`ADBMessageTransport`].
/// Structure is totally agnostic over which transport is truly used.
#[derive(Debug)]
pub(crate) struct ADBMessageDevice<T: ADBMessageTransport> {
    transport: T,
}

impl<T: ADBMessageTransport> ADBMessageDevice<T> {
    /// Instantiate a new [`ADBMessageDevice`]
    pub fn new(transport: T) -> Result<Self> {
        let mut message_device = Self { transport };
        message_device.connect()?;
        Ok(message_device)
    }

    pub(crate) fn get_transport_mut(&mut self) -> &mut T {
        &mut self.transport
    }

    /// Send initial connect
    fn connect(&mut self) -> Result<()> {
        self.get_transport_mut().connect()?;

        let message = ADBTransportMessage::try_new(
            MessageCommand::Cnxn,
            0x0100_0000,
            1_048_576,
            format!("host::{}\0", env!("CARGO_PKG_NAME")).as_bytes(),
        )?;

        self.get_transport_mut().write_message(message)?;

        let message = self.get_transport_mut().read_message()?;

        match message.header().command() {
            MessageCommand::Cnxn => {
                log::debug!("Connection established");
                Ok(())
            }
            MessageCommand::Auth => Err(RustADBError::ADBRequestFailed(
                "Device requires authentication, which is not supported".to_string(),
            )),
            _ => Err(RustADBError::WrongResponseReceived(
                "Expected CNXN command".to_string(),
                message.header().command().to_string(),
            )),
        }
    }

    pub(crate) fn open_session(&mut self, cmd: &ADBLocalCommand) -> Result<ADBSession<T>> {
        let mut rng = rand::rng();
        let local_id: u32 = rng.random();

        let message = ADBTransportMessage::try_new(
            MessageCommand::Open,
            local_id,
            0,
            cmd.to_string().as_bytes(),
        )?;
        self.transport.write_message(message)?;

        let response = self.transport.read_message()?;

        if response.header().command() != MessageCommand::Okay {
            return Err(RustADBError::ADBRequestFailed(format!(
                "Open session failed: got {} in respone instead of OKAY",
                response.header().command()
            )));
        }

        if response.header().arg1() != local_id {
            return Err(RustADBError::ADBRequestFailed(format!(
                "Open session failed: respones used {} for our local_id instead of {local_id}",
                response.header().arg1()
            )));
        }

        Ok(ADBSession::new(
            self.transport.clone(),
            local_id,
            response.header().arg0(),
        ))
    }
}

impl<T: ADBMessageTransport> Drop for ADBMessageDevice<T> {
    fn drop(&mut self) {
        let _ = self.get_transport_mut().disconnect();
    }
}
