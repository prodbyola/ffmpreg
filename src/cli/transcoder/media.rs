use crate::core::packet::Packet;
use crate::core::{Decoder, Encoder};
use crate::message::Result;

pub struct Transcoder {
	pub decoder: Box<dyn Decoder>,
	pub encoder: Box<dyn Encoder>,
}

impl Transcoder {
	pub fn new(decoder: Box<dyn Decoder>, encoder: Box<dyn Encoder>) -> Self {
		Self { decoder, encoder }
	}

	pub fn transcode(&mut self, packet: Packet) -> Result<Vec<Packet>> {
		let mut packets = Vec::new();
		if let Some(frame) = self.decoder.decode(packet)? {
			if let Some(encoded_packet) = self.encoder.encode(frame)? {
				packets.push(encoded_packet);
			}
		}
		Ok(packets)
	}

	pub fn flush(&mut self) -> Result<Vec<Packet>> {
		let mut packets = Vec::new();

		while let Some(frame) = self.decoder.flush()? {
			if let Some(encoded_packet) = self.encoder.encode(frame)? {
				packets.push(encoded_packet);
			}
		}

		while let Some(packet) = self.encoder.flush()? {
			packets.push(packet);
		}

		Ok(packets)
	}
}
