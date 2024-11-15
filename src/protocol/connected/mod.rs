use crate::{export_modules, misc::BinaryStream};

export_modules!(
	connection_request
/* 	connection_accepted, */
/* 	new_incoming */
);

pub trait FramePacket {
	const ID: u8;

	fn deserialize(stream: &mut BinaryStream) -> Option<Self> where Self: Sized;
	fn serialize(&self, stream: &mut BinaryStream);
}