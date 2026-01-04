use crate::container::raw::RawPcmFormat;
use crate::core::Muxer;
use crate::core::packet::Packet;
use crate::core::stream::{self, Stream, StreamKind};
use crate::core::time::Time;
use crate::io::{MediaSeek, MediaWrite, Result, WritePrimitives};

pub struct RawPcmMuxer<W: MediaWrite + MediaSeek> {
	writer: W,
	#[allow(dead_code)]
	format: RawPcmFormat,
	streams: stream::Streams,
	data_size: u32,
}

impl<W: MediaWrite + MediaSeek> RawPcmMuxer<W> {
	pub fn new(writer: W, format: RawPcmFormat) -> Result<Self> {
		let codec_name = format.to_codec_string().to_string();
		let time = Time::new(1, format.sample_rate);
		let mut streams = stream::Streams::new_empty();
		let stream = Stream::new(0, 0, StreamKind::Audio, codec_name, time);

		streams.add(stream);

		Ok(Self { writer, format, streams, data_size: 0 })
	}

	pub fn write_packet(&mut self, packet: Packet) -> Result<()> {
		self.writer.write_all(&packet.data)?;
		self.data_size += packet.data.len() as u32;
		Ok(())
	}

	pub fn finalize(&mut self) -> Result<()> {
		self.writer.flush()?;
		Ok(())
	}
}

impl<W: MediaWrite + MediaSeek> Muxer for RawPcmMuxer<W> {
	fn streams(&self) -> &stream::Streams {
		&self.streams
	}
	fn write(&mut self, packet: Packet) -> Result<()> {
		self.write_packet(packet)
	}
	fn finalize(&mut self) -> Result<()> {
		self.finalize()
	}
}
