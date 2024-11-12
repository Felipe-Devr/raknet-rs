use std::{io::{Seek, Write}, time::Duration};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use super::{Packet, MAGIC};

pub struct UnconnectedPing {
	pub timestamp: Duration,
	pub guid: u64
}

impl Packet for UnconnectedPing {
	const ID: u8 = 0x01;

	fn serialize(&self, buffer: &mut Vec<u8>) {
		buffer.write_u8(UnconnectedPing::ID).expect("Failed to write packet id.");
		buffer.write_u64::<LittleEndian>(self.timestamp.as_secs()).expect("Failed to write timestamp");
		buffer.write(&MAGIC).expect("Failed to write magic.");
		buffer.write_u64::<LittleEndian>(self.guid).expect("Failed to write guid");
	}

	fn deserialize(buffer: &mut std::io::Cursor<Vec<u8>>) -> Option<Self>  {
		buffer.read_u8().unwrap(); // Skip packet id.
		let timestamp: Duration = Duration::from_secs(buffer.read_u64::<LittleEndian>().unwrap());
		buffer.seek_relative(16).unwrap(); // Skip MAGIC.
		let guid: u64 = buffer.read_u64::<LittleEndian>().unwrap();

		Some(UnconnectedPing { timestamp, guid })
	}
}