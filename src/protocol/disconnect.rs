use crate::misc::BinaryStream;

use super::Packet;

pub struct Disconnect {}

impl Packet for Disconnect {
	const ID: u8 = 0x15;

	fn deserialize(_: &mut BinaryStream) -> Option<Self> {
        Some(Disconnect {})
    }

	fn serialize(&self, buff: &mut BinaryStream) {
		buff.write_u8(Self::ID);
	}
}