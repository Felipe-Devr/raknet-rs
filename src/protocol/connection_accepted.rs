use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::misc::{Address, AddressVersion};

use super::Packet;



pub struct ConnectionRequestAccepted {
	pub client_address: Address,
	pub request_time: u64,
	pub time: u64
}

impl Packet for ConnectionRequestAccepted {
	const ID: u8 = 0x10;

	fn deserialize(buffer: &mut std::io::Cursor<Vec<u8>>) -> Option<Self> {
		buffer.read_u8().expect("Failed to read packet id.");
		let client_address = Address::deserialize(buffer)?;	
		buffer.read_u8().unwrap();

		for _ in 0..9 {
			Address::deserialize(buffer);
		}
        let request_time = buffer.read_u64::<LittleEndian>().unwrap();
		let time = buffer.read_u64::<LittleEndian>().unwrap();

		Some(ConnectionRequestAccepted {
            client_address,
            request_time,
            time
        })
	}

	fn serialize(&self, buffer: &mut Vec<u8>) {
		buffer.write_u8(ConnectionRequestAccepted::ID).expect("Failed to write packet id.");
		self.client_address.serialize(buffer);
		buffer.write_u16::<LittleEndian>(0).unwrap();

		// Unknown use, works with this ip.
		let address = Address {
			ip: String::from("255.255.255.255"),
			port: 19132,
			version: AddressVersion::IPv4
		};

		// Write x10 address
		for _ in 0..9 {
			address.serialize(buffer);
		}

		buffer.write_u64::<LittleEndian>(self.request_time).unwrap();
        buffer.write_u64::<LittleEndian>(self.time).unwrap();
	}
}