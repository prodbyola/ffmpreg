use crate::core::{Encoder, Frame, Packet, Timebase};
use std::io::Result;

pub struct PcmEncoder {
	timebase: Timebase,
}

impl PcmEncoder {
	pub fn new(timebase: Timebase) -> Self {
		Self { timebase }
	}
}

impl Encoder for PcmEncoder {
	fn encode(&mut self, frame: Frame) -> Result<Option<Packet>> {
		let packet = Packet::new(frame.data, 0, self.timebase).with_pts(frame.pts);
		Ok(Some(packet))
	}

	fn flush(&mut self) -> Result<Option<Packet>> {
		Ok(None)
	}
}
