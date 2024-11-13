use std::net::{SocketAddr, IpAddr};

use super::{BinaryStream, Endianness};

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
    pub fn from(sockaddr: &SocketAddr) -> Address {
        Address {
            version: match sockaddr.ip() {
                IpAddr::V4(_) => AddressVersion::IPv4,
                IpAddr::V6(_) => AddressVersion::IPv6,
            },
            ip: sockaddr.ip().to_string(),
            port: sockaddr.port(),
        }
    }

    pub fn serialize(&self, buffer: &mut BinaryStream) {
        match self.version {
            AddressVersion::IPv4 => {
                buffer.write_u8(4);
                let octates: Vec<u8> = self
                    .ip
                    .split('.')
                    .map(|x| x.parse::<u8>().unwrap())
                    .collect();

				for octate in octates {
					buffer.write_u8(octate);
				}
				buffer.write_u16(self.port, Endianness::BigEndian);
            }
            AddressVersion::IPv6 => {
                buffer.write_u8(6);
                /*  buffer.write_u8(16).unwrap();
                buffer.write_all(self.ip.as_bytes()).unwrap();
                buffer.write_u16::<BigEndian>(self.port).unwrap(); */
            }
        }
    }

	pub fn deserialize(buffer: &mut BinaryStream) -> Option<Self> {
        let version = buffer.read_u8().unwrap();
        
		match version {
			4 | 0 => {
				let mut octets: Vec<u8> = Vec::new();

				for _ in 0..4 {
					octets.push(buffer.read_u8().unwrap());
				}                
				let port = buffer.read_u16(Endianness::BigEndian).unwrap();
			
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
