use std::fmt::Error;

use crate::{
    misc::{BinaryStream, Endianness},
    protocol::Reliability,
};

use super::u24::u24;

#[derive(Debug)]
pub struct Frame {
    pub reliability: Reliability,
    pub length: u16,
    pub reliable_index: Option<u32>,
    pub sequenced_frame_idx: Option<u32>,
    pub ordered_frame_idx: Option<u32>,
    pub ordered_channel: Option<u8>,
    pub compound_size: Option<i32>,
    pub compound_id: Option<u16>,
    pub index: Option<i32>,
    pub body: Vec<u8>,
}

pub enum BitFlags {
    Ack = 0x40,
    Nak = 0x20,
    Split = 0x10,
}

impl Frame {
    pub fn deserialize(buffer: &mut BinaryStream) -> Result<Frame, Error> {
        let flags = buffer.read_u8().unwrap();
        let length: u16 =
            (buffer.read_u16(Endianness::BigEndian).unwrap() as f32 / 8.0).ceil() as u16;

        let reliability = Reliability::from((flags & 0xe0) >> 5);
        let splitted = (flags & BitFlags::Split as u8) != 0;
        let reliable_index = if reliability.reliable() {
            Some(buffer.read_u24(Endianness::LittleEndian).unwrap().to_u32())
        } else {
            None
        };

        let mut sequenced_frame_idx = None;
        let mut ordered_frame_idx = None;
        let mut ordered_channel = None;
        let mut compound_size = None;
        let mut compound_id = None;
        let mut index = None;

        if reliability.sequenced() {
            sequenced_frame_idx = Some(buffer.read_u24(Endianness::LittleEndian).unwrap().to_u32());
        }

        if reliability.ordered() {
            ordered_frame_idx = Some(buffer.read_u24(Endianness::LittleEndian).unwrap().to_u32());
            ordered_channel = Some(buffer.read_u8().unwrap());
        }

        if splitted {
            compound_size = Some(buffer.read_i32(Endianness::BigEndian).unwrap());
            compound_id = Some(buffer.read_u16(Endianness::BigEndian).unwrap());
            index = Some(buffer.read_i32(Endianness::BigEndian).unwrap());
        }
        let body = buffer.read(length as usize).unwrap();

        Ok(Frame {
            reliability,
            length,
            reliable_index,
            sequenced_frame_idx,
            ordered_frame_idx,
            ordered_channel,
            compound_size,
            compound_id,
            index,
            body,
        })
    }

    pub fn serialize(&self, buffer: &mut BinaryStream) {
        let flags = (self.reliability.clone() as u8) << 5;
        let flag_modifier = if self.compound_size.is_some() && self.compound_size.unwrap() > 0 {
            BitFlags::Split as u8
        } else {
            0
        };

        buffer.write_u8(flags | flag_modifier);
        buffer.write_u16((self.body.len() as u16) << 3, Endianness::BigEndian);

        if self.reliability.reliable() {
            buffer.write_u24(
                u24::from_u32(self.reliable_index.unwrap()),
                Endianness::LittleEndian,
            );
        }

        if self.reliability.sequenced() {
            buffer.write_u24(
                u24::from_u32(self.sequenced_frame_idx.unwrap()),
                Endianness::LittleEndian,
            );
        }

        if self.reliability.ordered() {
            buffer.write_u24(
                u24::from_u32(self.ordered_frame_idx.unwrap()),
                Endianness::LittleEndian,
            );
            buffer.write_u8(self.ordered_channel.unwrap());
        }

        if flag_modifier == BitFlags::Split as u8 {
            buffer.write_i32(self.compound_size.unwrap(), Endianness::BigEndian);
            buffer.write_u16(self.compound_id.unwrap(), Endianness::BigEndian);
            buffer.write_i32(self.index.unwrap(), Endianness::BigEndian);
        }
        buffer.write(&self.body);
    }
}
