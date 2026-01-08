use super::common::Pipeline;
use crate::cli::transcoder::media;
use crate::cli::utils;
use crate::codecs::audio::pcm::{PcmDecoder, PcmEncoder};
use crate::container::{self, raw, wav};
use crate::core::{Demuxer, Muxer};
use crate::io::{Error, File};
use crate::message::Result;

pub fn run(pipeline: Pipeline) -> Result<()> {
	let input_extension = utils::get_extension(&pipeline.input)?;
	let mut format = wav::WavFormat::default();
	let mut metadata = None;

	if input_extension == container::WAV {
		let file = File::open(&pipeline.input)?;
		let demuxer = wav::WavDemuxer::new(file)?;
		format = demuxer.format();
		metadata = Some(demuxer.metadata().clone())
	}

	let mut target_format = format;
	if let Some(codec) = &pipeline.audio.codec {
		target_format.apply_codec(codec).map_err(Error::invalid_data)?;
	}

	let output_file = File::create(&pipeline.output)?;
	let mut muxer = wav::WavMuxer::new(output_file, target_format)?;
	muxer.with_metadata(metadata);

	let mut demuxer = create_demuxer(&pipeline.input, &input_extension, format)?;
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

fn create_demuxer(path: &str, extension: &str, format: wav::WavFormat) -> Result<Box<dyn Demuxer>> {
	let file = File::open(path)?;
	if extension == container::WAV {
		return Ok(Box::new(wav::WavDemuxer::new(file)?));
	}
	let demuxer = raw::RawPcmDemuxer::new(file, format.to_raw_format())?;
	Ok(Box::new(demuxer))
}

fn create_transcoder(format: wav::WavFormat, target_format: wav::WavFormat) -> media::Transcoder {
	let decoder = PcmDecoder::new_from_metadata(&format);

	if format.audio_format() != target_format.audio_format() {
		let encoder = PcmEncoder::new(target_format.sample_rate);
		let encoder = encoder.with_target_format(target_format.audio_format());
		return media::Transcoder::new(Box::new(decoder), Box::new(encoder));
	}

	let encoder = PcmEncoder::new(target_format.sample_rate);
	media::Transcoder::new(Box::new(decoder), Box::new(encoder))
}
