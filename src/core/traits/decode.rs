use crate::{
	core::{frame::Frame, packet::Packet},
	io::Result,
};

pub trait Decoder {
	fn decode(&mut self, packet: Packet) -> Result<Option<Frame>>;
	fn flush(&mut self) -> Result<Option<Frame>>;
}
