use std::time::Duration;

use crate::misc::{BinaryStream, Endianness};

use super::{Packet, MAGIC};

pub struct UnconnectedPing {
    pub timestamp: Duration,
    pub guid: u64,
}

impl Packet for UnconnectedPing {
    const ID: u8 = 0x01;

    fn serialize(&self, buffer: &mut BinaryStream) {
        buffer.write_u8(UnconnectedPing::ID);
        buffer.write_u64(self.timestamp.as_millis() as u64, Endianness::BigEndian);
        buffer.write(&MAGIC);
        buffer.write_u64(self.guid, Endianness::BigEndian);
    }

    fn deserialize(buffer: &mut BinaryStream) -> Option<Self> {
        let timestamp: Duration =
            Duration::from_secs(buffer.read_u64(Endianness::BigEndian).unwrap());
        buffer.advance(16); // Skip MAGIC.
        let guid: u64 = buffer.read_u64(Endianness::BigEndian).unwrap();

        Some(UnconnectedPing { timestamp, guid })
    }
}
