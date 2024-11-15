

use crate::{export_modules, misc::BinaryStream};

export_modules!(
    open_reply1,
    open_reply2,
    open_request2,
    open_request1,
    unconnected_pong,
    unconnected_ping,
	disconnect,
    incompatible_protocol,
    connected,
    frame_set
);

pub trait Packet {
    const ID: u8;

    fn deserialize(buffer: &mut BinaryStream) -> Option<Self>
    where
        Self: Sized;
    fn serialize(&self, buffer: &mut BinaryStream);
}

pub const MAGIC: [u8; 16] = [
    0x00, 0xFF, 0xFF, 0x00, 0xFE, 0xFE, 0xFE, 0xFE, 0xFD, 0xFD, 0xFD, 0xFD, 0x12, 0x34, 0x56, 0x78,
];
