use super::Packet;
use crate::misc::{Address, AddressVersion, BinaryStream, Endianness};
use std::time::Duration;

pub struct ConnectionRequestAccepted {
    pub client_address: Address,
    pub request_time: Duration,
    pub time: Duration,
}

impl Packet for ConnectionRequestAccepted {
    const ID: u8 = 0x10;

    fn deserialize(buffer: &mut BinaryStream) -> Option<Self> {
        let client_address = Address::deserialize(buffer)?;
        buffer.read_u8().unwrap();

        for _ in 0..9 {
            Address::deserialize(buffer);
        }
        let request_time = Duration::from_millis(buffer.read_u64(Endianness::BigEndian).unwrap());
        let time = Duration::from_millis(buffer.read_u64(Endianness::BigEndian).unwrap());

        Some(ConnectionRequestAccepted {
            client_address,
            request_time,
            time,
        })
    }

    fn serialize(&self, buffer: &mut BinaryStream) {
        buffer.write_u8(ConnectionRequestAccepted::ID);
        self.client_address.serialize(buffer);
        buffer.write_u16(0, Endianness::BigEndian);

        // Unknown use, works with this ip.
        let address = Address {
            ip: String::from("255.255.255.255"),
            port: 19132,
            version: AddressVersion::IPv4,
        };

        // Write x10 address
        for _ in 0..9 {
            address.serialize(buffer);
        }

        buffer.write_u64(self.request_time.as_millis() as u64, Endianness::BigEndian);
        buffer.write_u64(self.time.as_millis() as u64, Endianness::BigEndian);
    }
}
