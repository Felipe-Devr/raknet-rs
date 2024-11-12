use std::io::Cursor;

use byteorder::WriteBytesExt;

use super::Packet;

pub struct Disconnect {}

impl Packet for Disconnect {
	const ID: u8 = 0x15;

	fn deserialize(_: &mut Cursor<Vec<u8>>) -> Option<Self> {
        Some(Disconnect {})
    }

	fn serialize(&self, buff: &mut Vec<u8>) {
		buff.write_u8(Self::ID).expect("Failed to write packet id.");
	}
}