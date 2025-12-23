use ffmpreg::codecs::{AdpcmDecoder, AdpcmEncoder, PcmDecoder, PcmEncoder};
use ffmpreg::container::WavFormat;
use ffmpreg::core::{Decoder, Encoder, Frame, FrameAudio, Timebase};

fn create_test_format() -> WavFormat {
	WavFormat { channels: 1, sample_rate: 44100, bit_depth: 16 }
}

fn create_stereo_format() -> WavFormat {
	WavFormat { channels: 2, sample_rate: 44100, bit_depth: 16 }
}

fn generate_sine_wave(samples: usize, frequency: f32, sample_rate: u32) -> Vec<i16> {
	let mut data = Vec::with_capacity(samples);
	for i in 0..samples {
		let t = i as f32 / sample_rate as f32;
		let sample = (2.0 * std::f32::consts::PI * frequency * t).sin() * 16000.0;
		data.push(sample as i16);
	}
	data
}

#[test]
fn test_pcm_roundtrip_identity() {
	let format = create_test_format();
	let timebase = Timebase::new(1, format.sample_rate);

	let original_samples = generate_sine_wave(1024, 440.0, format.sample_rate);
	let original_data: Vec<u8> = original_samples.iter().flat_map(|s| s.to_le_bytes()).collect();

	let audio = FrameAudio::new(original_data.clone(), format.sample_rate, format.channels);
	let frame = Frame::new_audio(audio, timebase, 0);

	let mut encoder = PcmEncoder::new(timebase);
	let packet = encoder.encode(frame).unwrap().unwrap();

	let mut decoder = PcmDecoder::new(format);
	let decoded_frame = decoder.decode(packet).unwrap().unwrap();

	assert_eq!(decoded_frame.audio().unwrap().data, original_data);
}

#[test]
fn test_pcm_roundtrip_stereo() {
	let format = create_stereo_format();
	let timebase = Timebase::new(1, format.sample_rate);

	let left = generate_sine_wave(512, 440.0, format.sample_rate);
	let right = generate_sine_wave(512, 880.0, format.sample_rate);

	let mut interleaved = Vec::with_capacity(left.len() * 2);
	for (l, r) in left.iter().zip(right.iter()) {
		interleaved.push(*l);
		interleaved.push(*r);
	}

	let original_data: Vec<u8> = interleaved.iter().flat_map(|s| s.to_le_bytes()).collect();
	let audio = FrameAudio::new(original_data.clone(), format.sample_rate, format.channels);
	let frame = Frame::new_audio(audio, timebase, 0);

	let mut encoder = PcmEncoder::new(timebase);
	let packet = encoder.encode(frame).unwrap().unwrap();

	let mut decoder = PcmDecoder::new(format);
	let decoded_frame = decoder.decode(packet).unwrap().unwrap();

	assert_eq!(decoded_frame.audio().unwrap().data, original_data);
	assert_eq!(decoded_frame.audio().unwrap().channels, 2);
}

#[test]
fn test_adpcm_roundtrip_approximate() {
	let format = create_test_format();
	let timebase = Timebase::new(1, format.sample_rate);

	let original_samples = generate_sine_wave(256, 440.0, format.sample_rate);
	let original_data: Vec<u8> = original_samples.iter().flat_map(|s| s.to_le_bytes()).collect();

	let audio = FrameAudio::new(original_data, format.sample_rate, format.channels);
	let frame = Frame::new_audio(audio, timebase, 0);

	let mut encoder = AdpcmEncoder::new(timebase, format.channels);
	let packet = encoder.encode(frame).unwrap().unwrap();

	let mut decoder = AdpcmDecoder::new(format);
	let decoded_frame = decoder.decode(packet).unwrap().unwrap();

	let audio_frame = decoded_frame.audio().unwrap();
	assert_eq!(audio_frame.nb_samples, 256);
	assert_eq!(audio_frame.channels, 1);

	let decoded_samples: Vec<i16> =
		audio_frame.data.chunks(2).map(|c| i16::from_le_bytes([c[0], c[1]])).collect();

	for (i, (orig, dec)) in original_samples.iter().zip(decoded_samples.iter()).enumerate() {
		let diff = (*orig as i32 - *dec as i32).abs();
		assert!(
			diff < 5000,
			"Sample {} differs too much: original={}, decoded={}, diff={}",
			i,
			orig,
			dec,
			diff
		);
	}
}

#[test]
fn test_multi_frame_roundtrip() {
	let format = create_test_format();
	let timebase = Timebase::new(1, format.sample_rate);

	let mut encoder = PcmEncoder::new(timebase);
	let mut decoder = PcmDecoder::new(format);

	for frame_idx in 0..5 {
		let samples = generate_sine_wave(512, 440.0 + frame_idx as f32 * 100.0, format.sample_rate);
		let data: Vec<u8> = samples.iter().flat_map(|s| s.to_le_bytes()).collect();
		let pts = frame_idx as i64 * 512;

		let audio = FrameAudio::new(data.clone(), format.sample_rate, format.channels);
		let frame = Frame::new_audio(audio, timebase, 0).with_pts(pts);

		let packet = encoder.encode(frame).unwrap().unwrap();
		let decoded = decoder.decode(packet).unwrap().unwrap();

		assert_eq!(decoded.audio().unwrap().data, data);
		assert_eq!(decoded.pts, pts);
	}
}

#[test]
fn test_pcm_double_roundtrip() {
	let format = create_test_format();
	let timebase = Timebase::new(1, format.sample_rate);

	let original_samples = generate_sine_wave(256, 440.0, format.sample_rate);
	let original_data: Vec<u8> = original_samples.iter().flat_map(|s| s.to_le_bytes()).collect();

	let audio = FrameAudio::new(original_data.clone(), format.sample_rate, format.channels);
	let frame1 = Frame::new_audio(audio, timebase, 0);

	let mut encoder1 = PcmEncoder::new(timebase);
	let packet1 = encoder1.encode(frame1).unwrap().unwrap();

	let mut decoder1 = PcmDecoder::new(format);
	let decoded1 = decoder1.decode(packet1).unwrap().unwrap();

	let mut encoder2 = PcmEncoder::new(timebase);
	let packet2 = encoder2.encode(decoded1).unwrap().unwrap();

	let mut decoder2 = PcmDecoder::new(format);
	let decoded2 = decoder2.decode(packet2).unwrap().unwrap();

	assert_eq!(decoded2.audio().unwrap().data, original_data);
}

#[test]
fn test_adpcm_double_roundtrip() {
	let format = create_test_format();
	let timebase = Timebase::new(1, format.sample_rate);

	let original_samples = generate_sine_wave(256, 440.0, format.sample_rate);
	let original_data: Vec<u8> = original_samples.iter().flat_map(|s| s.to_le_bytes()).collect();

	let audio = FrameAudio::new(original_data.clone(), format.sample_rate, format.channels);
	let frame1 = Frame::new_audio(audio, timebase, 0);

	let mut encoder1 = AdpcmEncoder::new(timebase, format.channels);
	let packet1 = encoder1.encode(frame1).unwrap().unwrap();

	let mut decoder1 = AdpcmDecoder::new(format);
	let decoded1 = decoder1.decode(packet1).unwrap().unwrap();

	let mut encoder2 = AdpcmEncoder::new(timebase, format.channels);
	let packet2 = encoder2.encode(decoded1.clone()).unwrap().unwrap();

	let mut decoder2 = AdpcmDecoder::new(format);
	let decoded2 = decoder2.decode(packet2).unwrap().unwrap();

	assert_eq!(decoded2.audio().unwrap().nb_samples, decoded1.audio().unwrap().nb_samples);
}
