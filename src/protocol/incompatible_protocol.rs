use crate::misc::{BinaryStream, Endianness};

use super::{Packet, MAGIC};

pub struct IncompatibleProtocol {
    pub protocol: u8,
    pub server_guid: u64,
}

impl Packet for IncompatibleProtocol {
    const ID: u8 = 0x19;

    fn deserialize(buffer: &mut BinaryStream) -> Option<Self> {
        let protocol = buffer.read_u8().unwrap();
        buffer.advance(MAGIC.len()); // Skip magic bytes.
        let server_guid = buffer.read_u64(Endianness::BigEndian).unwrap();

        Some(IncompatibleProtocol {
            protocol,
            server_guid,
        })
    }

    fn serialize(&self, buffer: &mut BinaryStream) {
        buffer.write_u8(IncompatibleProtocol::ID);
        buffer.write_u8(self.protocol);
        buffer.write(&MAGIC);
        buffer.write_u64(self.server_guid, Endianness::BigEndian);
    }
}
