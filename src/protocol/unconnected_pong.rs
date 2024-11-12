use std::{io::{Read, Seek, Write}, time::Duration};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use super::{Packet, MAGIC};

#[derive(Debug)]
pub struct UnconnectedPong {
	pub timestamp: Duration,
	pub server_guid: u64,
	pub id: String
}

impl Packet for UnconnectedPong {
	const ID: u8 = 0x1c;

	fn deserialize(buffer: &mut std::io::Cursor<Vec<u8>>) -> Option<Self> {
		buffer.read_u8().unwrap();
		let timestamp = Duration::from_secs(buffer.read_u64::<BigEndian>().unwrap() as u64);
		let server_guid = buffer.read_u64::<BigEndian>().unwrap();
		buffer.seek_relative(MAGIC.len() as i64).unwrap(); // Skup Magic
		buffer.read_u16::<BigEndian>().expect("Failed to read server identity string.");
		let mut id = String::new();

		buffer.read_to_string(&mut id).expect("Failed to read server identity");

		Some(UnconnectedPong {
			timestamp,
            server_guid,
            id
		})
	}

	fn serialize(&self, buffer: &mut Vec<u8>) {
		buffer.write_u8(UnconnectedPong::ID).expect("Failed to write packet id");
		buffer.write_u64::<BigEndian>(self.timestamp.as_millis() as u64).expect("Failed to write timestamp.");
		buffer.write_u64::<BigEndian>(self.server_guid).expect("Failed to write server guid.");
		buffer.write(&MAGIC).unwrap();
		buffer.write_u16::<BigEndian>(self.id.len() as u16).unwrap();
		buffer.extend_from_slice(self.id.as_bytes());
	}
}