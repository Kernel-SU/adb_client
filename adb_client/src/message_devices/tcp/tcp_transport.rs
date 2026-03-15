use crate::{
    Result, RustADBError,
    adb_transport::ADBTransport,
    message_devices::{
        adb_message_transport::ADBMessageTransport,
        adb_transport_message::{ADBTransportMessage, ADBTransportMessageHeader},
    },
};
use std::{
    io::{Read, Write},
    net::{Shutdown, SocketAddr, TcpStream},
    sync::{Arc, Mutex},
    time::Duration,
};

/// Transport running over TCP.
#[derive(Clone, Debug)]
pub struct TcpTransport {
    address: SocketAddr,
    connection: Option<Arc<Mutex<TcpStream>>>,
}

impl TcpTransport {
    /// Instantiate a new [`TcpTransport`]
    pub fn new<A: Into<SocketAddr>>(address: A) -> Self {
        Self {
            address: address.into(),
            connection: None,
        }
    }

    fn get_connection(&self) -> Result<Arc<Mutex<TcpStream>>> {
        self.connection
            .as_ref()
            .ok_or(RustADBError::IOError(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "not connected",
            )))
            .cloned()
    }
}

impl ADBTransport for TcpTransport {
    fn connect(&mut self) -> Result<()> {
        let stream = TcpStream::connect(self.address)?;
        stream.set_nodelay(true)?;
        self.connection = Some(Arc::new(Mutex::new(stream)));
        Ok(())
    }

    fn disconnect(&mut self) -> Result<()> {
        log::debug!("disconnecting...");
        if let Some(conn) = &self.connection {
            let lock = conn.lock()?;
            let _ = lock.shutdown(Shutdown::Both);
        }
        Ok(())
    }
}

impl ADBMessageTransport for TcpTransport {
    fn read_message_with_timeout(
        &mut self,
        read_timeout: std::time::Duration,
    ) -> Result<ADBTransportMessage> {
        let conn_lock = self.get_connection()?;
        let mut conn = conn_lock.lock()?;

        conn.set_read_timeout(Some(read_timeout))?;

        let mut data = [0; 24];
        let mut total_read = 0;
        loop {
            total_read += conn.read(&mut data[total_read..])?;
            if total_read == data.len() {
                break;
            }
        }

        let header = ADBTransportMessageHeader::try_from(data)?;

        if header.data_length() != 0 {
            let mut msg_data = vec![0_u8; header.data_length() as usize];
            let mut total_read = 0;
            loop {
                total_read += conn.read(&mut msg_data[total_read..])?;
                if total_read == msg_data.capacity() {
                    break;
                }
            }

            let message = ADBTransportMessage::from_header_and_payload(header, msg_data);

            if !message.check_message_integrity() {
                return Err(RustADBError::InvalidIntegrity(
                    ADBTransportMessageHeader::compute_crc32(message.payload()),
                    message.header().data_crc32(),
                ));
            }

            return Ok(message);
        }

        Ok(ADBTransportMessage::from_header_and_payload(header, vec![]))
    }

    fn write_message_with_timeout(
        &mut self,
        message: ADBTransportMessage,
        write_timeout: Duration,
    ) -> Result<()> {
        let message_bytes = message.header().as_bytes();
        let conn_lock = self.get_connection()?;
        let mut conn = conn_lock.lock()?;

        conn.set_write_timeout(Some(write_timeout))?;

        let mut total_written = 0;
        loop {
            total_written += conn.write(&message_bytes[total_written..])?;
            if total_written == message_bytes.len() {
                conn.flush()?;
                break;
            }
        }

        let payload = message.into_payload();
        if !payload.is_empty() {
            let mut total_written = 0;
            loop {
                total_written += conn.write(&payload[total_written..])?;
                if total_written == payload.len() {
                    conn.flush()?;
                    break;
                }
            }
        }

        Ok(())
    }
}
