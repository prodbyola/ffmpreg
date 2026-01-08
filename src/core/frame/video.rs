#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoFormat {
	RGB24,
	RGBA32,
	YUV420,
	YUV422,
	YUV444,
	GRAY8,
}

impl VideoFormat {
	pub fn bytes_per_pixel(&self) -> Option<usize> {
		match self {
			VideoFormat::RGB24 => Some(3),
			VideoFormat::RGBA32 => Some(4),
			VideoFormat::YUV444 => Some(3),
			VideoFormat::GRAY8 => Some(1),
			VideoFormat::YUV420 | VideoFormat::YUV422 => None,
		}
	}

	pub fn expected_size(&self, width: u32, height: u32) -> usize {
		match self {
			VideoFormat::RGB24 => (width * height * 3) as usize,
			VideoFormat::RGBA32 => (width * height * 4) as usize,
			VideoFormat::YUV444 => (width * height * 3) as usize,
			VideoFormat::GRAY8 => (width * height) as usize,
			VideoFormat::YUV420 => ((width * height * 3) / 2) as usize,
			VideoFormat::YUV422 => (width * height * 2) as usize,
		}
	}
}

#[derive(Debug, Clone)]
pub struct FrameVideo {
	pub data: Vec<u8>,
	pub width: u32,
	pub height: u32,
	pub format: VideoFormat,
	pub keyframe: bool,
}

impl FrameVideo {
	pub fn new(data: Vec<u8>, width: u32, height: u32, format: VideoFormat, keyframe: bool) -> Self {
		Self { data, width, height, format, keyframe }
	}

	pub fn expected_size(&self) -> usize {
		self.format.expected_size(self.width, self.height)
	}

	pub fn is_valid(&self) -> bool {
		self.data.len() == self.expected_size()
	}
}
