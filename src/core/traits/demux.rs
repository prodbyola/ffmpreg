use crate::core::Packet;
use std::io::Result;

pub trait Demuxer {
	fn read_packet(&mut self) -> Result<Option<Packet>>;
	fn stream_count(&self) -> usize;
}
