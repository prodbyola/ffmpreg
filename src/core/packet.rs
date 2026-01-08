use crate::core::time::Time;

#[derive(Debug, Clone)]
pub struct Packet {
	pub data: Vec<u8>,
	pub pts: i64,
	pub dts: i64,
	pub time: Time,
	pub stream_id: u32,
	pub keyframe: bool,
	pub discard: bool,
}

impl Packet {
	pub fn new(data: Vec<u8>, stream_id: u32, time: Time) -> Self {
		Self { data, pts: 0, dts: 0, time, stream_id, keyframe: false, discard: false }
	}

	pub fn with_pts(mut self, pts: i64) -> Self {
		self.pts = pts;
		self
	}

	pub fn with_dts(mut self, dts: i64) -> Self {
		self.dts = dts;
		self
	}

	pub fn with_keyframe(mut self, keyframe: bool) -> Self {
		self.keyframe = keyframe;
		self
	}

	pub fn size(&self) -> u32 {
		self.data.len() as u32
	}

	pub fn is_empty(&self) -> bool {
		self.data.is_empty()
	}
}
