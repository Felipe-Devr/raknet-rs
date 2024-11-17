use super::{Packet, MAGIC};
use crate::misc::{BinaryStream, Endianness};
use std::time::Duration;

#[derive(Debug)]
pub struct UnconnectedPong {
    pub timestamp: Duration,
    pub server_guid: u64,
    pub id: String,
}

impl Packet for UnconnectedPong {
    const ID: u8 = 0x1c;

    fn deserialize(buffer: &mut BinaryStream) -> Option<Self> {
        let timestamp = Duration::from_secs(buffer.read_u64(Endianness::BigEndian).unwrap());
        let server_guid = buffer.read_u64(Endianness::BigEndian).unwrap();
        buffer.advance(MAGIC.len()); // Skip Magic

        let id = buffer.read_string().unwrap().to_string();

        Some(UnconnectedPong {
            timestamp,
            server_guid,
            id,
        })
    }

    fn serialize(&self, buffer: &mut BinaryStream) {
        buffer.write_u8(UnconnectedPong::ID);
        buffer.write_u64(self.timestamp.as_millis() as u64, Endianness::BigEndian);
        buffer.write_u64(self.server_guid, Endianness::BigEndian);
        buffer.write(&MAGIC);
        buffer.write_string(self.id.as_str());
    }
}
