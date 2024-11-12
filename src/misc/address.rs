use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub enum AddressVersion {
    IPv4,
    IPv6,
}

pub struct Address {
    pub version: AddressVersion,
    pub ip: String,
    pub port: u16,
}

impl Address {
    pub fn serialize(&self, buffer: &mut Vec<u8>) {
        match self.version {
            AddressVersion::IPv4 => {
                buffer.write_u8(4).unwrap();
                let octates: Vec<u8> = self
                    .ip
                    .split('.')
                    .map(|x| x.parse::<u8>().unwrap())
                    .collect();

				for octate in octates {
					buffer.write_u8(octate).unwrap();
				}
				buffer.write_u16::<LittleEndian>(self.port).unwrap();
            }
            AddressVersion::IPv6 => {
                buffer.write_u8(6).unwrap();
                /*  buffer.write_u8(16).unwrap();
                buffer.write_all(self.ip.as_bytes()).unwrap();
                buffer.write_u16::<LittleEndian>(self.port).unwrap(); */
            }
        }
    }

	pub fn deserialize(buffer: &mut std::io::Cursor<Vec<u8>>) -> Option<Self> {
		let version = buffer.read_u8().unwrap();

		match version {
			4 => {
				let mut octets: Vec<u8> = Vec::new();

				for _ in 0..3 {
					octets.push(buffer.read_u8().unwrap());
				}
				let port = buffer.read_u16::<LittleEndian>().unwrap();
				
				Some(Address {
                    version: AddressVersion::IPv4,
                    ip: octets.iter().map(|x| format!("{}", x)).collect::<Vec<String>>().join("."),
                    port,
                })
			}

			6 => {
				// TODO: Implement IPv6 deserialization
                None
			}
			_ => None, // Unsupported address version
		}
	}
}
