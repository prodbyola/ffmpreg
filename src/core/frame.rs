use super::time::Timebase;

#[derive(Debug, Clone)]
pub struct Frame {
	pub data: Vec<u8>,
	pub pts: i64,
	pub timebase: Timebase,
	pub sample_rate: u32,
	pub channels: u8,
	pub nb_samples: usize,
}

impl Frame {
	pub fn new(
		data: Vec<u8>,
		timebase: Timebase,
		sample_rate: u32,
		channels: u8,
		nb_samples: usize,
	) -> Self {
		Self { data, pts: 0, timebase, sample_rate, channels, nb_samples }
	}

	pub fn with_pts(mut self, pts: i64) -> Self {
		self.pts = pts;
		self
	}

	pub fn size(&self) -> usize {
		self.data.len()
	}

	pub fn is_empty(&self) -> bool {
		self.data.is_empty()
	}
}
