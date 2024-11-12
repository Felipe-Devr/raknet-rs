use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::misc::Address;

use super::Packet;

pub struct NewIncomingConnection {
	pub server_address: Address,
	pub internal_address: Vec<Address>,
	pub incoming_timestamp: u64,
	pub server_timestamp: u64
}

impl Packet for NewIncomingConnection {
	const ID: u8 = 0x13;

	fn deserialize(buffer: &mut std::io::Cursor<Vec<u8>>) -> Option<Self> {
		buffer.read_u8().expect("Failed to read packet id.");
		let server_address = Address::deserialize(buffer)?;
		let mut internal_address: Vec<Address> = Vec::with_capacity(20);

		for _ in 0..20 {
            internal_address.push(Address::deserialize(buffer)?);
        }
		let incoming_timestamp = buffer.read_u64::<byteorder::BigEndian>().unwrap();
		let server_timestamp = buffer.read_u64::<byteorder::BigEndian>().unwrap();

		Some(NewIncomingConnection {
			server_address,
            internal_address,
            incoming_timestamp,
            server_timestamp,
		})
	}

	fn serialize(&self, buffer: &mut Vec<u8>) {
		buffer.write_u8(NewIncomingConnection::ID).expect("Failed to write packet id.");
		self.server_address.serialize(buffer);

		for address in &self.internal_address {
			address.serialize(buffer);
		}
        buffer.write_u64::<byteorder::BigEndian>(self.incoming_timestamp).unwrap();
		buffer.write_u64::<byteorder::BigEndian>(self.server_timestamp).unwrap();
	}
}