use crate::misc::{Address, BinaryStream, Endianness};

use super::{Packet, MAGIC};

pub struct OpenRequest2 {
    pub address: Address,
    pub cookie: Option<i32>,
    pub supports_security: bool,
    pub mtu: u16,
    pub client_guid: u64,
}

impl Packet for OpenRequest2 {
    const ID: u8 = 0x07;

    // TODO: Implement cookie and security
    fn deserialize(buffer: &mut BinaryStream) -> Option<Self> {
        buffer.advance(MAGIC.len());
        let address = Address::deserialize(buffer).unwrap();
        /* let cookie = buffer.read_i32(Endianness::BigEndian).unwrap(); */
        /* let supports_security = buffer.read_bool().unwrap(); */
        let mtu = buffer.read_u16(Endianness::BigEndian).unwrap();
        let client_guid = buffer.read_u64(Endianness::BigEndian).unwrap();

        Some(OpenRequest2 {
            address,
            cookie: None,
            supports_security: false,
            mtu,
            client_guid,
        })
    }

    fn serialize(&self, buffer: &mut BinaryStream) {
        buffer.write_u8(OpenRequest2::ID);
        buffer.write(&MAGIC);

        self.address.serialize(buffer);

        if self.cookie.is_some() {
            buffer.write_i32(self.cookie.unwrap(), Endianness::BigEndian);
        }
        buffer.write_bool(self.supports_security); // Supports security.
        buffer.write_u16(self.mtu, Endianness::BigEndian);
        buffer.write_u64(self.client_guid, Endianness::BigEndian);
    }
}
