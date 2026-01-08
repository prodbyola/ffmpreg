use crate::core::packet::Packet;
use crate::core::stream::Streams;
use crate::message::Result;

pub trait Demuxer {
	fn streams(&self) -> &Streams;
	fn read_packet(&mut self) -> Result<Option<Packet>>;

	fn read_audio_packet(&mut self) -> Result<Option<Packet>> {
		while let Some(packet) = self.read_packet()? {
			let stream = self.streams().get(packet.stream_id);
			if stream.map(|pkt| pkt.audio_kind()).unwrap_or(false) {
				return Ok(Some(packet));
			}
		}

		Ok(None)
	}

	fn read_video_packet(&mut self) -> Result<Option<Packet>> {
		while let Some(packet) = self.read_packet()? {
			let stream = self.streams().get(packet.stream_id);
			if stream.map(|pkt| pkt.video_kind()).unwrap_or(false) {
				return Ok(Some(packet));
			}
		}
		Ok(None)
	}

	fn read_subtitle_packet(&mut self) -> Result<Option<Packet>> {
		while let Some(packet) = self.read_packet()? {
			let stream = self.streams().get(packet.stream_id);
			if stream.map(|pkt| pkt.subtitle_kind()).unwrap_or(false) {
				return Ok(Some(packet));
			}
		}
		Ok(None)
	}
}
