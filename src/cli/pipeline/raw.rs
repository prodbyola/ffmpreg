use super::common::Pipeline;
use crate::cli::transcoder::media;
use crate::cli::utils;
use crate::codecs::audio::pcm::{PcmDecoder, PcmEncoder};
use crate::container::{self, raw, wav};
use crate::core::{Demuxer, Muxer};
use crate::io::{Error, File, Result};

pub fn run(pipeline: Pipeline) -> Result<()> {
	let input_extension = utils::get_extension(&pipeline.input)?;
	let mut format = raw::RawPcmFormat::default();

	if input_extension == container::WAV {
		let file = File::open(&pipeline.input)?;
		let demuxer = wav::WavDemuxer::new(file)?;
		format = demuxer.format().to_raw_format();
	}

	let mut target_format = format;
	if let Some(codec) = &pipeline.audio.codec {
		target_format.apply_codec(codec).map_err(Error::invalid_data)?;
	}

	let output_file = File::create(&pipeline.output)?;
	let mut muxer = raw::RawPcmMuxer::new(output_file, target_format)?;

	let mut demuxer = create_demuxer(&pipeline.input, format, &input_extension)?;
	let mut transcoder = create_transcoder(format, target_format);

	while let Some(packet) = demuxer.read_packet()? {
		for output_packet in transcoder.transcode(packet)? {
			muxer.write(output_packet)?;
		}
	}

	for packet in transcoder.flush()? {
		muxer.write(packet)?;
	}

	muxer.finalize()
}

fn create_demuxer(
	path: &str,
	format: raw::RawPcmFormat,
	extension: &str,
) -> Result<Box<dyn Demuxer>> {
	let file = File::open(path)?;
	if extension == container::WAV {
		let demuxer = wav::WavDemuxer::new(file)?;
		return Ok(Box::new(demuxer));
	}
	let demuxer = raw::RawPcmDemuxer::new(file, format)?;
	Ok(Box::new(demuxer))
}

fn create_transcoder(format: raw::RawPcmFormat, target: raw::RawPcmFormat) -> media::Transcoder {
	let decoder = PcmDecoder::new(format.sample_rate, format.channels, format.bytes_per_sample());

	if format.audio_format() != target.audio_format() {
		let encoder = PcmEncoder::new(target.sample_rate);
		let encoder = encoder.with_target_format(target.audio_format());
		return media::Transcoder::new(Box::new(decoder), Box::new(encoder));
	}

	let encoder = PcmEncoder::new(target.sample_rate);
	media::Transcoder::new(Box::new(decoder), Box::new(encoder))
}
