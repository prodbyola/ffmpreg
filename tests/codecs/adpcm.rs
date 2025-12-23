use ffmpreg::codecs::{AdpcmDecoder, AdpcmEncoder};
use ffmpreg::container::WavFormat;
use ffmpreg::core::{Decoder, Encoder, Frame, FrameAudio, Packet, Timebase};

fn create_mono_format() -> WavFormat {
	WavFormat { channels: 1, sample_rate: 44100, bit_depth: 16 }
}

// fn create_stereo_format() -> WavFormat {
// 	WavFormat { channels: 2, sample_rate: 44100, bit_depth: 16 }
// }

#[test]
fn test_adpcm_decoder_basic() {
	let format = create_mono_format();
	let mut decoder = AdpcmDecoder::new(format);

	let timebase = Timebase::new(1, 44100);
	let packet = Packet::new(vec![0x00, 0x00, 0x00, 0x00], 0, timebase);

	let frame = decoder.decode(packet).unwrap().unwrap();

	if let Some(audio) = frame.audio() {
		assert_eq!(audio.channels, 1);
		assert_eq!(audio.sample_rate, 44100);
		assert_eq!(audio.nb_samples, 8);
	} else {
		panic!("Expected audio frame");
	}
}

#[test]
fn test_adpcm_decoder_output_size() {
	let format = create_mono_format();
	let mut decoder = AdpcmDecoder::new(format);

	let timebase = Timebase::new(1, 44100);
	let packet = Packet::new(vec![0x12, 0x34], 0, timebase);

	let frame = decoder.decode(packet).unwrap().unwrap();

	assert_eq!(frame.size(), 8);
}

#[test]
fn test_adpcm_decoder_preserves_pts() {
	let format = create_mono_format();
	let mut decoder = AdpcmDecoder::new(format);

	let timebase = Timebase::new(1, 44100);
	let packet = Packet::new(vec![0x00], 0, timebase).with_pts(5000);

	let frame = decoder.decode(packet).unwrap().unwrap();
	assert_eq!(frame.pts, 5000);
}

#[test]
fn test_adpcm_encoder_basic() {
	let timebase = Timebase::new(1, 44100);
	let mut encoder = AdpcmEncoder::new(timebase, 1);

	let samples: Vec<i16> = vec![0, 100, -100, 500, -500, 1000, -1000, 2000];
	let data: Vec<u8> = samples.iter().flat_map(|s| s.to_le_bytes()).collect();
	let audio = FrameAudio::new(data, 44100, 1);
	let frame = Frame::new_audio(audio, timebase, 0);

	let packet = encoder.encode(frame).unwrap().unwrap();

	assert!(packet.data.len() < 16);
}

#[test]
fn test_adpcm_encoder_stereo() {
	let timebase = Timebase::new(1, 44100);
	let mut encoder = AdpcmEncoder::new(timebase, 2);

	let samples: Vec<i16> = vec![0, 100, -100, 500, -500, 1000, -1000, 2000];
	let data: Vec<u8> = samples.iter().flat_map(|s| s.to_le_bytes()).collect();
	let audio = FrameAudio::new(data, 44100, 2);
	let frame = Frame::new_audio(audio, timebase, 0).with_pts(7777);

	let packet = encoder.encode(frame).unwrap().unwrap();

	assert_eq!(packet.pts, 7777);
	assert!(packet.data.len() > 0);
}

#[test]
fn test_adpcm_roundtrip() {
	let format = create_mono_format();
	let timebase = Timebase::new(1, 44100);
	let mut encoder = AdpcmEncoder::new(timebase, 1);
	let mut decoder = AdpcmDecoder::new(format);

	let samples: Vec<i16> = vec![0, 100, -100, 500, -500, 1000, -1000, 2000];
	let data: Vec<u8> = samples.iter().flat_map(|s| s.to_le_bytes()).collect();
	let audio = FrameAudio::new(data, 44100, 1);
	let frame = Frame::new_audio(audio, timebase, 0).with_pts(1000);

	let packet = encoder.encode(frame).unwrap().unwrap();
	let decoded = decoder.decode(packet).unwrap().unwrap();

	if let Some(audio) = decoded.audio() {
		assert_eq!(audio.nb_samples, 8);
		assert_eq!(audio.sample_rate, 44100);
	} else {
		panic!("Expected audio frame");
	}

	assert_eq!(decoded.pts, 1000);
}

#[test]
fn test_adpcm_decoder_empty() {
	let format = create_mono_format();
	let mut decoder = AdpcmDecoder::new(format);

	let timebase = Timebase::new(1, 44100);
	let packet = Packet::new(vec![], 0, timebase);

	let result = decoder.decode(packet).unwrap();
	assert!(result.is_none());
}
