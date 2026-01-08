use super::RawPcmFormat;
use crate::core::packet::Packet;
use crate::core::{Demuxer, stream, time};
use crate::io::MediaRead;
use crate::message::Result;

pub struct RawPcmDemuxer<R: MediaRead> {
	reader: R,
	format: RawPcmFormat,
	streams: stream::Streams,
	data_remaining: Option<u64>,
	packet_count: u64,
	sample_position: u64,
}

impl<R: MediaRead> RawPcmDemuxer<R> {
	const CHUNK_SIZE_LIMIT: usize = 65536;

	pub fn new(reader: R, format: RawPcmFormat) -> Result<Self> {
		let codec_name = format.to_codec_string().to_string();
		let time = time::Time::new(1, format.sample_rate);
		let stream = stream::Stream::new(0, 0, stream::StreamKind::Audio, codec_name, time);
		let streams = stream::Streams::new(vec![stream]);

		Ok(Self { reader, format, streams, data_remaining: None, packet_count: 0, sample_position: 0 })
	}

	pub fn read_packet(&mut self) -> Result<Option<Packet>> {
		let block_align = self.format.block_align() as u64;
		let max_chunk = (Self::CHUNK_SIZE_LIMIT as u64 / block_align) * block_align;
		let chunk_size = max_chunk as usize;
		let mut data = vec![0u8; chunk_size];
		let bytes_read = self.reader.read(&mut data)?;

		if bytes_read == 0 {
			return Ok(None);
		}

		data.truncate(bytes_read);

		if let Some(ref mut remaining) = self.data_remaining {
			*remaining -= bytes_read as u64;
		}

		let time = time::Time::new(1, self.format.sample_rate);
		let packet = Packet::new(data, 0, time).with_pts(self.sample_position as i64);

		self.sample_position += (bytes_read / self.format.bytes_per_frame()) as u64;
		self.packet_count += 1;

		Ok(Some(packet))
	}

	pub fn read_audio_packet(&mut self) -> Result<Option<Packet>> {
		self.read_packet()
	}

	pub fn format(&self) -> RawPcmFormat {
		self.format
	}
}

impl<R: MediaRead> Demuxer for RawPcmDemuxer<R> {
	fn streams(&self) -> &stream::Streams {
		&self.streams
	}
	fn read_packet(&mut self) -> Result<Option<Packet>> {
		self.read_packet()
	}
}
