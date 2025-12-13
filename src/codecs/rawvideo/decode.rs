use crate::container::Y4mFormat;
use crate::core::{Decoder, Frame, Packet};
use crate::io::IoResult;

pub struct RawVideoDecoder {
	format: Y4mFormat,
}

impl RawVideoDecoder {
	pub fn new(format: Y4mFormat) -> Self {
		Self { format }
	}
}

impl Decoder for RawVideoDecoder {
	fn decode(&mut self, packet: Packet) -> IoResult<Option<Frame>> {
		let frame = Frame::new(packet.data, packet.timebase, self.format.framerate_num, 1, 1)
			.with_pts(packet.pts);
		Ok(Some(frame))
	}

	fn flush(&mut self) -> IoResult<Option<Frame>> {
		Ok(None)
	}
}
