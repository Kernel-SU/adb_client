use crate::Result;

pub trait BinaryEncodable {
    fn encode(&self) -> Vec<u8>;
}

/// Internal trait representing binary decoding capabilities.
pub trait BinaryDecodable {
    /// Decode binary data into a struct.
    fn decode(data: &[u8]) -> Result<Self>
    where
        Self: Sized;
}
