use crate::core::Transform;
use crate::core::frame::Frame;
use crate::message::Result;

pub struct Normalize {}

impl Normalize {}

impl Transform for Normalize {
	fn apply(&mut self, mut frame: Frame) -> Result<Frame> {
		if let Some(_audio_frame) = frame.audio_mut() {}
		Ok(frame)
	}

	fn name(&self) -> &'static str {
		"normalize"
	}
}
