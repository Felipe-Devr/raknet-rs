use crate::misc::{BinaryStream, Endianness};

use super::{Packet, MAGIC};

#[derive(Debug)]
pub struct OpenReply1 {
    pub server_guid: u64,
    pub use_security: bool,
    pub cookie: Option<i32>,
    pub mtu: u16,
}

impl Packet for OpenReply1 {
    const ID: u8 = 0x06;

    fn deserialize(stream: &mut BinaryStream) -> Option<Self> {
        stream.advance(16); // Skip Packet MAGIC

        let server_guid = stream.read_u64(Endianness::BigEndian).unwrap(); // Read server guid
        let use_security = stream.read_bool().unwrap(); // Read use security
        let mut cookie: Option<i32> = None;

        if use_security {
            cookie = Some(stream.read_i32(Endianness::BigEndian).unwrap()); // Read cookie.
        }
        let mtu = stream.read_u16(Endianness::BigEndian).unwrap(); // Read MTU.

        Some(OpenReply1 {
            server_guid,
            use_security,
            cookie,
            mtu,
        })
    }

    fn serialize(&self, buffer: &mut BinaryStream) {
        buffer.write_u8(OpenReply1::ID); // Write Packet id
        buffer.write(&MAGIC); // Write MAGIC
        buffer.write_u64(self.server_guid, Endianness::BigEndian); // Write server guid

        buffer.write_bool(self.use_security); // Write use security flag
        if self.use_security {
            buffer
                .write_i32(self.cookie.unwrap(), Endianness::BigEndian); // Write cookie
        }
        buffer.write_u16(self.mtu, Endianness::BigEndian); // Write MTU
    }
}
