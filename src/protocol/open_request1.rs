use crate::misc::BinaryStream;

use super::{Packet, MAGIC};

#[derive(Debug)]
pub struct OpenRequest1 {
    pub protocol_version: u8,
    pub mtu: u16,
}

impl Packet for OpenRequest1 {
    const ID: u8 = 0x05;

    fn deserialize(buffer: &mut BinaryStream) -> Option<Self> {
        buffer.advance(MAGIC.len());
        let protocol = buffer.read_u8().unwrap();
        let mut mtu: Vec<u8> = vec![];
        buffer.read_to_end(&mut mtu);

        Some(OpenRequest1 {
            protocol_version: protocol,
            mtu: mtu.len() as u16,
        })
    }

    fn serialize(&self, buffer: &mut BinaryStream) {
        buffer.write_u8(OpenRequest1::ID); // Write Packet ID
        buffer.write(&MAGIC); // Write MAGIC
        buffer.write_u8(self.protocol_version); // Write Protocol Version
        buffer.expand(buffer.size() + (self.mtu as usize - 46), 0x00); // Write the MTU
    }
}
