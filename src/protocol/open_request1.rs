
use std::io::{Cursor, Read, Seek, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};

use super::{Packet, MAGIC};


pub struct OpenRequest1 {
	pub protocol_version: u8,
	pub mtu: u16
}

impl Packet for OpenRequest1 {
	const ID: u8 = 0x05;

	fn deserialize(buffer: &mut Cursor<Vec<u8>>) -> Option<Self> {
		buffer.read_u8().expect("Failed to read packet id");
		buffer.seek_relative(MAGIC.len() as i64).unwrap();
		let protocol = buffer.read_u8().unwrap();
		let mut mtu: Vec<u8> = vec![];
		buffer.read_to_end(&mut mtu).expect("Failed to read mtu.");
		
		Some(
			OpenRequest1 {
                protocol_version: protocol,
                mtu: mtu.len() as u16
            }
		)
	}

	fn serialize(&self, buffer: &mut Vec<u8>) {
		buffer.write_u8(OpenRequest1::ID).unwrap(); // Write Packet ID
		buffer.write(&MAGIC).unwrap(); // Write MAGIC
		buffer.write_u8(self.protocol_version).unwrap(); // Write Protocol Version
		buffer.resize(buffer.len() + (self.mtu as usize - 46), 0x00 ); // Write the MTU
	}
}