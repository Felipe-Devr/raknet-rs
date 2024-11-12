use std::time::Duration;

use byteorder::{ReadBytesExt, WriteBytesExt};

use super::Packet;



pub struct ConnectionRequest {
	pub guid: u64,
	pub timestamp: Duration,
	pub use_security: bool
}

impl Packet for ConnectionRequest {
	const ID: u8 = 0x09;

	fn deserialize(buffer: &mut std::io::Cursor<Vec<u8>>) -> Option<Self> {
		buffer.read_u8().unwrap();
		let guid = buffer.read_u64::<byteorder::BigEndian>().unwrap();
		let timestamp = Duration::from_secs(buffer.read_u64::<byteorder::BigEndian>().unwrap());
		let use_security = buffer.read_u8().map(|b| b == 1).unwrap();

		Some(ConnectionRequest { guid, timestamp, use_security })
	}

	fn serialize(&self, buffer: &mut Vec<u8>) {
		buffer.write_u8(ConnectionRequest::ID).expect("Failed to write packet id");
		buffer.write_u64::<byteorder::BigEndian>(self.guid).expect("Failed to write guid.");
		buffer.write_u64::<byteorder::BigEndian>(self.timestamp.as_millis() as u64).expect("Failed to write packet timestamp");
		buffer.write_u8(self.use_security as u8).expect("Failed to write use_security");
	}
}