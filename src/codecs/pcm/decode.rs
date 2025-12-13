use crate::container::WavFormat;
use crate::core::{Decoder, Frame, Packet};
use crate::io::IoResult;

pub struct PcmDecoder {
	format: WavFormat,
}

impl PcmDecoder {
	pub fn new(format: WavFormat) -> Self {
		Self { format }
	}
}

impl Decoder for PcmDecoder {
	fn decode(&mut self, packet: Packet) -> IoResult<Option<Frame>> {
		let nb_samples = packet.size() / self.format.bytes_per_frame();

		let frame = Frame::new(
			packet.data,
			packet.timebase,
			self.format.sample_rate,
			self.format.channels,
			nb_samples,
		)
		.with_pts(packet.pts);

		Ok(Some(frame))
	}

	fn flush(&mut self) -> IoResult<Option<Frame>> {
		Ok(None)
	}
}
