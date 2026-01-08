pub mod audio;
pub mod subtitle;
pub mod video;

pub use audio::*;
pub use subtitle::*;
pub use video::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameKind {
	Audio,
	Video,
	Subtitle,
}

#[derive(Debug, Clone)]
pub enum FrameData {
	Audio(FrameAudio),
	Video(FrameVideo),
	Subtitle(FrameSubtitle),
}

#[derive(Debug, Clone)]
pub struct Frame {
	pub pts: i64,
	pub dts: Option<i64>,
	pub stream_id: u32,
	pub data: FrameData,
}

impl Frame {
	pub fn new_audio(audio: FrameAudio, stream_id: u32) -> Self {
		Self { pts: 0, dts: None, data: FrameData::Audio(audio), stream_id }
	}

	pub fn new_video(video: FrameVideo, stream_id: u32) -> Self {
		Self { pts: 0, dts: None, data: FrameData::Video(video), stream_id }
	}

	pub fn new_subtitle(subtitle: FrameSubtitle, stream_id: u32) -> Self {
		Self { pts: 0, dts: None, data: FrameData::Subtitle(subtitle), stream_id }
	}

	pub fn with_pts(mut self, pts: i64) -> Self {
		self.pts = pts;
		self
	}

	#[inline]
	pub fn audio(&self) -> Option<&FrameAudio> {
		match &self.data {
			FrameData::Audio(audio) => Some(audio),
			_ => None,
		}
	}

	#[inline]
	pub fn audio_mut(&mut self) -> Option<&mut FrameAudio> {
		match &mut self.data {
			FrameData::Audio(audio) => Some(audio),
			_ => None,
		}
	}

	#[inline]
	pub fn video(&self) -> Option<&FrameVideo> {
		match &self.data {
			FrameData::Video(video) => Some(video),
			_ => None,
		}
	}
	#[inline]
	pub fn video_mut(&mut self) -> Option<&mut FrameVideo> {
		match &mut self.data {
			FrameData::Video(video) => Some(video),
			_ => None,
		}
	}

	#[inline]
	pub fn kind(&self) -> FrameKind {
		match &self.data {
			FrameData::Audio(_) => FrameKind::Audio,
			FrameData::Video(_) => FrameKind::Video,
			FrameData::Subtitle(_) => FrameKind::Subtitle,
		}
	}

	#[inline]
	pub fn audio_kind(&self) -> bool {
		matches!(self.kind(), FrameKind::Audio)
	}

	#[inline]
	pub fn video_kind(&self) -> bool {
		matches!(self.kind(), FrameKind::Video)
	}

	#[inline]
	pub fn subtitle_kind(&self) -> bool {
		matches!(self.kind(), FrameKind::Subtitle)
	}

	pub fn size(&self) -> usize {
		match &self.data {
			FrameData::Audio(a) => a.data.len(),
			FrameData::Video(v) => v.data.len(),
			FrameData::Subtitle(s) => s.data.len(),
		}
	}

	pub fn is_empty(&self) -> bool {
		self.size() == 0
	}
}
