use crate::misc::{u24::u24, BinaryStream, Endianness, Frame};

use super::Packet;

#[derive(PartialEq, Clone, Eq, Debug)]
pub enum Reliability {
    Unrealiable,
    UnrealiableSequenced,
    Reliable,
    ReliableOrdered,
    ReliableSequenced,
    UnrealiableAck,
    ReliableAck,
    ReliableOrderedAck,
    Invalid,
}

impl Reliability {
    pub fn from(byte: u8) -> Reliability {
        match byte {
            0 => Reliability::Unrealiable,
            1 => Reliability::UnrealiableSequenced,
            2 => Reliability::Reliable,
            3 => Reliability::ReliableOrdered,
            4 => Reliability::ReliableSequenced,
            5 => Reliability::UnrealiableAck,
            6 => Reliability::ReliableAck,
            7 => Reliability::ReliableOrderedAck,
            _ => Reliability::Invalid,
        }
    }

    pub fn sequenced(&self) -> bool {
        matches!(
            self,
            Reliability::UnrealiableSequenced | Reliability::ReliableSequenced
        )
    }

    pub fn ordered(&self) -> bool {
        matches!(
            self,
            Reliability::ReliableOrdered | Reliability::ReliableOrderedAck
        )
    }

    pub fn reliable(&self) -> bool {
        matches!(
            self,
            Reliability::Reliable
                | Reliability::ReliableAck
                | Reliability::ReliableOrderedAck
                | Reliability::ReliableOrdered
                | Reliability::ReliableSequenced
        )
    }
}

#[derive(Debug)]
pub struct FrameSetPacket {
    pub sequence: u32,
    pub frames: Vec<Frame>, // Assuming Frame is defined elsewhere
}

impl Packet for FrameSetPacket {
    const ID: u8 = 0xF; // UNUSED

    fn deserialize(buffer: &mut BinaryStream) -> Option<Self> {
        let sequence = buffer
            .read_u24(Endianness::LittleEndian)
            .expect("Failed to read sequence number.");
        let mut frames: Vec<Frame> = vec![];

        while !buffer.finished() {
            let frame = Frame::deserialize(buffer).unwrap();
            frames.push(frame);
        }

        Some(FrameSetPacket {
            sequence: sequence.to_u32(),
            frames, // Assuming Frame is defined elsewhere
        })
    }

    fn serialize(&self, buffer: &mut BinaryStream) {
        buffer.write_u24(u24::from_u32(self.sequence), Endianness::LittleEndian);

        for frame in &self.frames {
            frame.serialize(buffer);
        }
    }
}
