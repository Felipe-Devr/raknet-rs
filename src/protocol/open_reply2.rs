

use crate::misc::{BinaryStream, Address, Endianness};

use super::{Packet, MAGIC};

pub struct OpenReply2 {
	pub server_guid: u64,
	pub client_address: Address,
	pub mtu: u16,
	pub encryption_enabled: bool
}

impl Packet for OpenReply2 {
	const ID: u8 = 0x08;

	fn deserialize(buffer: &mut BinaryStream) -> Option<Self> {
		buffer.advance(16);
		let server_guid = buffer.read_u64(Endianness::BigEndian).unwrap();
		let client_address = Address::deserialize(buffer)?;
		let mtu = buffer.read_u16(Endianness::BigEndian).unwrap();
		let encryption_enabled = buffer.read_bool().unwrap();

		Some(OpenReply2 {
            server_guid,
            client_address,
            mtu,
            encryption_enabled,
        })
	}

	fn serialize(&self, buffer: &mut BinaryStream) {
		buffer.write_u8(OpenReply2::ID);
		buffer.write(&MAGIC);
		buffer.write_u64(self.server_guid, Endianness::BigEndian);
		self.client_address.serialize(buffer);
		buffer.write_u16(self.mtu, Endianness::BigEndian);
		buffer.write_bool(self.encryption_enabled);
	}
}