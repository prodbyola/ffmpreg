use crate::{container::wav::WavFormat, core::frame::Channels, error, message::Result};

#[derive(Debug)]
pub struct WavHeader {
	pub channels: Channels,
	pub sample_rate: u32,
	pub byte_rate: u32,
	pub block_align: u16,
	pub bits_per_sample: u16,
	pub format_code: u16,
}

impl WavHeader {
	pub fn to_format(&self) -> WavFormat {
		WavFormat {
			channels: self.channels,
			sample_rate: self.sample_rate,
			bit_depth: self.bits_per_sample,
			format_code: self.format_code,
		}
	}

	pub fn validate(&self) -> Result<()> {
		if self.channels.count() == 0 {
			return Err(error!("channels must be non-zero"));
		}
		if self.sample_rate == 0 {
			return Err(error!("sample rate must be non-zero"));
		}

		match self.format_code {
			1 | 3 => self.validate_pcm_bits(),
			0x11 => self.validate_ima_adpcm(),
			code => Err(error!("audio format code {} is not supported", code)),
		}
	}

	pub fn validate_pcm_bits(&self) -> Result<()> {
		if self.bits_per_sample == 0 {
			return Err(error!("bits per sample must be non-zero"));
		}
		if self.bits_per_sample % 8 != 0 {
			return Err(error!("bits per sample must be multiple of 8"));
		}
		Ok(())
	}

	pub fn validate_ima_adpcm(&self) -> Result<()> {
		if self.bits_per_sample != 4 {
			return Err(error!("IMA ADPCM must have 4 bits per sample"));
		}
		Ok(())
	}
}
