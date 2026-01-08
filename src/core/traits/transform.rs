use crate::{core::frame::Frame, message::Result};

pub trait Transform: Send {
	fn apply(&mut self, frame: Frame) -> Result<Frame>;
	fn name(&self) -> &'static str;
}
