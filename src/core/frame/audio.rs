#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
	PCM16,
	PCM24,
	PCM32,
	FLAC,
	AAC,
	OPUS,
	ADPCM,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channels {
	Mono,
	Stereo,
	Quad,
	Surround,      // 5.1
	SevenPointOne, // 7.1
	Custom(u8),
}

impl Channels {
	pub fn count(&self) -> u8 {
		match self {
			Channels::Mono => 1,
			Channels::Stereo => 2,
			Channels::Quad => 4,
			Channels::Surround => 6,
			Channels::SevenPointOne => 8,
			Channels::Custom(c) => *c,
		}
	}

	pub fn name(&self) -> String {
		match self {
			Channels::Mono => "mono".into(),
			Channels::Stereo => "stereo".into(),
			Channels::Quad => "quad".into(),
			Channels::Surround => "5.1".into(),
			Channels::SevenPointOne => "7.1".into(),
			Channels::Custom(c) => format!("{} channels", c),
		}
	}

	/// Convert from u8 channel count to Channels enum
	pub fn from_count(count: u8) -> Self {
		match count {
			1 => Channels::Mono,
			2 => Channels::Stereo,
			4 => Channels::Quad,
			6 => Channels::Surround,
			8 => Channels::SevenPointOne,
			c => Channels::Custom(c),
		}
	}
}

impl AudioFormat {
	pub fn bytes_per_sample(&self) -> Option<usize> {
		match self {
			AudioFormat::PCM16 => Some(2),
			AudioFormat::PCM24 => Some(3),
			AudioFormat::PCM32 => Some(4),
			AudioFormat::FLAC | AudioFormat::AAC | AudioFormat::OPUS | AudioFormat::ADPCM => None,
		}
	}
}

#[derive(Debug, Clone)]
pub struct FrameAudio {
	pub data: Vec<u8>,
	pub sample_rate: u32,
	pub channels: Channels,
	pub nb_samples: usize,
	pub format: AudioFormat,
}

impl FrameAudio {
	pub fn new(data: Vec<u8>, sample_rate: u32, channels: Channels, format: AudioFormat) -> Self {
		let nb_samples = match format.bytes_per_sample() {
			Some(bps) => data.len() / (channels.count() as usize * bps),
			None => 0,
		};
		Self { data, sample_rate, channels, nb_samples, format }
	}

	pub fn frame_size(&self) -> Option<usize> {
		self.format.bytes_per_sample().map(|bps| bps * self.channels.count() as usize * self.nb_samples)
	}

	pub fn with_nb_samples(mut self, nb_samples: usize) -> Self {
		self.nb_samples = nb_samples;
		self
	}

	pub fn with_format(mut self, format: AudioFormat) -> Self {
		self.format = format;
		self
	}

	pub fn is_compressed(&self) -> bool {
		self.format.bytes_per_sample().is_none()
	}
}
