use super::FramePacket;
use crate::misc::{BinaryStream, Endianness};
use std::time::Duration;

pub struct ConnectionRequest {
    pub guid: u64,
    pub timestamp: Duration,
    pub use_security: bool,
}

impl FramePacket for ConnectionRequest {
    const ID: u8 = 0x09;

    fn deserialize(buffer: &mut BinaryStream) -> Option<ConnectionRequest> {
        let guid = buffer.read_u64(Endianness::BigEndian).unwrap();
        let timestamp = Duration::from_secs(buffer.read_u64(Endianness::BigEndian).unwrap());
        let use_security = buffer.read_bool().unwrap();

        Some(ConnectionRequest {
            guid,
            timestamp,
            use_security,
        })
    }

    fn serialize(&self, buffer: &mut BinaryStream) {
        buffer.write_u8(ConnectionRequest::ID);
        buffer.write_u64(self.guid, Endianness::BigEndian);
        buffer.write_u64(self.timestamp.as_millis() as u64, Endianness::BigEndian);
        buffer.write_bool(self.use_security);
    }
}
