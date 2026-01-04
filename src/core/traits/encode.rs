use crate::core::frame::Frame;
use crate::core::packet::Packet;
use crate::io::Result;

pub trait Encoder {
	fn encode(&mut self, frame: Frame) -> Result<Option<Packet>>;
	fn flush(&mut self) -> Result<Option<Packet>>;
}
