use std::io::{Seek, SeekFrom, Write};

use byteorder::{BigEndian, WriteBytesExt};

use crate::misc::Address;

use super::{Packet, MAGIC};

pub struct OpenRequest2 {
	pub address: Address,
	pub cookie: Option<i32>,
	pub supports_security: bool,
	pub mtu: u16,
	pub client_guid: i64
}

impl Packet for OpenRequest2 {
	const ID: u8 = 0x07;

	fn deserialize(buffer: &mut std::io::Cursor<Vec<u8>>) -> Option<Self> {
		buffer.seek(SeekFrom::Current(17)).unwrap();

		None
	}

	fn serialize(&self, buffer: &mut Vec<u8>) {
		buffer.write_u8(OpenRequest2::ID).unwrap();
		buffer.write(&MAGIC).unwrap();
		
		self.address.serialize(buffer);

		if self.cookie.is_some() {
			buffer.write_i32::<BigEndian>(self.cookie.unwrap()).unwrap();
		}
		buffer.write_u8(0).unwrap(); // Supports security.
		buffer.write_u16::<BigEndian>(self.mtu).unwrap();
		buffer.write_i64::<BigEndian>(self.client_guid).unwrap();
	}
}