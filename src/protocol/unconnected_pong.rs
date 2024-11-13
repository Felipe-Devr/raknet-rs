use std::time::Duration;
use crate::misc::{BinaryStream, Endianness};
use super::{Packet, MAGIC};

#[derive(Debug)]
pub struct UnconnectedPong {
	pub timestamp: Duration,
	pub server_guid: u64,
	pub id: String
}

impl Packet for UnconnectedPong {
	const ID: u8 = 0x1c;

	fn deserialize(buffer: &mut BinaryStream) -> Option<Self> {
		let timestamp = Duration::from_secs(buffer.read_u64(Endianness::BigEndian).unwrap() as u64);
		let server_guid = buffer.read_u64(Endianness::BigEndian).unwrap();
		buffer.advance(MAGIC.len()); // Skip Magic
		buffer.read_u16(Endianness::BigEndian).expect("Failed to read server identity string.");
		let id = buffer.read_string().unwrap();

		Some(UnconnectedPong {
			timestamp,
            server_guid,
            id
		})
	}

	fn serialize(&self, buffer: &mut BinaryStream) {
		buffer.write_u8(UnconnectedPong::ID);
		buffer.write_u64(self.timestamp.as_millis() as u64, Endianness::BigEndian);
		buffer.write_u64(self.server_guid, Endianness::BigEndian);
		buffer.write(&MAGIC);
		buffer.write_string(self.id.as_str());
	}
}