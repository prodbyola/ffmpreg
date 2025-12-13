use crate::core::{Encoder, Frame, Packet, Timebase};
use crate::io::IoResult;

pub struct PcmEncoder {
	timebase: Timebase,
}

impl PcmEncoder {
	pub fn new(timebase: Timebase) -> Self {
		Self { timebase }
	}
}

impl Encoder for PcmEncoder {
	fn encode(&mut self, frame: Frame) -> IoResult<Option<Packet>> {
		let packet = Packet::new(frame.data, 0, self.timebase).with_pts(frame.pts);
		Ok(Some(packet))
	}

	fn flush(&mut self) -> IoResult<Option<Packet>> {
		Ok(None)
	}
}
