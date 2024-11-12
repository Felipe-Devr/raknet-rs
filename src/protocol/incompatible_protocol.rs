use std::io::{Seek, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use super::{Packet, MAGIC};

pub struct IncompatibleProtocol {
	pub protocol: u8,
	pub server_guid: u64
}

impl Packet for IncompatibleProtocol {
	const ID: u8 = 0x19;

	fn deserialize(buffer: &mut std::io::Cursor<Vec<u8>>) -> Option<Self> {
		buffer.read_u8().expect("Failed to read packet id.");
		let protocol = buffer.read_u8().unwrap();
		buffer.seek_relative(MAGIC.len() as i64).unwrap(); // Skip magic bytes.
		let server_guid = buffer.read_u64::<BigEndian>().unwrap();

		Some(IncompatibleProtocol { protocol, server_guid })
	}

	fn serialize(&self, buffer: &mut Vec<u8>) {
		buffer.write_u8(IncompatibleProtocol::ID).expect("Failed to write packet id.");
		buffer.write_u8(self.protocol).expect("Failed to write protocol version.");
		buffer.write(&MAGIC).expect("Failed to write packet MAGIC.");
		buffer.write_u64::<BigEndian>(self.server_guid).expect("Failed to write server GUID.");
	}
}