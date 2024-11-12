use std::io::{Cursor, Seek, SeekFrom, Write};

use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};

use super::{Packet, MAGIC};

#[derive(Debug)]
pub struct OpenReply1 {
    pub server_guid: u64,
    pub use_security: Option<bool>,
    pub cookie: Option<i32>,
    pub mtu: u16,
}

impl Packet for OpenReply1 {
    const ID: u8 = 0x06;

    fn deserialize(cursor: &mut Cursor<Vec<u8>>) -> Option<Self> {
        cursor.seek(SeekFrom::Current(17)).unwrap(); // Skip Packet ID and MAGIC

        let server_guid = cursor.read_u64::<BigEndian>().unwrap(); // Read server guid
        let use_security = cursor.read_u8().map(|b| b == 1).unwrap(); // Read use security
        let mut cookie: Option<i32> = None;

        if use_security {
            cookie = Some(cursor.read_i32::<LittleEndian>().unwrap()); // Read cookie.
        }
        let mtu = cursor.read_u16::<BigEndian>().unwrap(); // Read MTU.

        Some(OpenReply1 {
            server_guid,
            use_security: Some(use_security),
            cookie,
            mtu,
        })
    }

    fn serialize(&self, buffer: &mut Vec<u8>) {
        buffer.write_u8(OpenReply1::ID).unwrap(); // Write Packet id
        buffer.write(&MAGIC).unwrap(); // Write MAGIC
        buffer.write_u64::<LittleEndian>(self.server_guid).unwrap(); // Write server guid

        if self.use_security.is_some() {
            buffer.write_u8(1).unwrap(); // Write use security flag
            buffer
                .write_i32::<LittleEndian>(self.cookie.unwrap())
                .unwrap(); // Write Cookie
        }
        buffer.write_u16::<LittleEndian>(self.mtu).unwrap(); // Write MTU
    }
}
