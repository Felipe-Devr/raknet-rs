
use crate::misc::{Address, BinaryStream, Endianness};
use super::FramePacket;

pub struct NewIncomingConnection {
	pub server_address: Address,
	pub internal_address: Vec<Address>,
	pub incoming_timestamp: u64,
	pub server_timestamp: u64
}

impl FramePacket for NewIncomingConnection {
	const ID: u8 = 0x13;

	fn deserialize(buffer: &mut BinaryStream) -> Option<Self> {
		let server_address = Address::deserialize(buffer)?;
		let mut internal_address: Vec<Address> = Vec::with_capacity(20);

		for _ in 0..20 {
            internal_address.push(Address::deserialize(buffer)?);
        }
		let incoming_timestamp = buffer.read_u64(Endianness::BigEndian).unwrap();
		let server_timestamp = buffer.read_u64(Endianness::BigEndian).unwrap();

		Some(NewIncomingConnection {
			server_address,
            internal_address,
            incoming_timestamp,
            server_timestamp,
		})
	}

	fn serialize(&self, buffer: &mut BinaryStream) {
		buffer.write_u8(NewIncomingConnection::ID);
		self.server_address.serialize(buffer);

		for address in &self.internal_address {
			address.serialize(buffer);
		}
        buffer.write_u64(self.incoming_timestamp, Endianness::BigEndian);
		buffer.write_u64(self.server_timestamp, Endianness::BigEndian);
	}
}