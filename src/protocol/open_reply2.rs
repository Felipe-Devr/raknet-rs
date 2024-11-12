use std::io::{Seek, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::misc::Address;

use super::{Packet, MAGIC};

pub struct OpenReply2 {
	server_guid: u64,
	client_address: Address,
	mtu: u16,
	encryption_enabled: bool
}

impl Packet for OpenReply2 {
	const ID: u8 = 0x08;

	fn deserialize(buffer: &mut std::io::Cursor<Vec<u8>>) -> Option<Self> {
		buffer.seek_relative(17).unwrap();
		let server_guid = buffer.read_u64::<LittleEndian>().unwrap();
		let client_address = Address::deserialize(buffer)?;
		let mtu = buffer.read_u16::<LittleEndian>().unwrap();
		let encryption_enabled = buffer.read_u8().map(|b| b == 1).unwrap();

		Some(OpenReply2 {
            server_guid,
            client_address,
            mtu,
            encryption_enabled,
        })
	}

	fn serialize(&self, buffer: &mut Vec<u8>) {
		buffer.write_u8(OpenReply2::ID).expect("Failed to write packet id.");
		buffer.write(&MAGIC).unwrap();
		buffer.write_u64::<LittleEndian>(self.server_guid).expect("Failed to write server_guid.");
		self.client_address.serialize(buffer);
		buffer.write_u16::<LittleEndian>(self.mtu).expect("Failed to write mtu.");
		buffer.write_u8(self.encryption_enabled as u8).unwrap();
	}
}