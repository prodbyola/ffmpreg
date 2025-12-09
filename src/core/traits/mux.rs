use crate::core::Packet;
use std::io::Result;

pub trait Muxer {
	fn write_packet(&mut self, packet: Packet) -> Result<()>;
	fn finalize(&mut self) -> Result<()>;
}
