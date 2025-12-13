pub mod gain;
pub mod normalize;

pub use gain::Gain;
pub use normalize::Normalize;

use crate::core::Transform;
use crate::io::{IoError, IoErrorKind, IoResult};

pub fn parse_transform(spec: &str) -> IoResult<Box<dyn Transform>> {
	let parts: Vec<&str> = spec.splitn(2, '=').collect();
	let name = parts[0];

	match name {
		"gain" => {
			let factor = parts
				.get(1)
				.ok_or_else(|| {
					IoError::with_message(IoErrorKind::InvalidData, "gain requires a value (e.g., gain=2.0)")
				})?
				.parse::<f32>()
				.map_err(|_| {
					IoError::with_message(IoErrorKind::InvalidData, "gain value must be a number")
				})?;
			Ok(Box::new(Gain::new(factor)))
		}
		"normalize" => {
			let peak = parts.get(1).map(|v| v.parse::<f32>().unwrap_or(0.95)).unwrap_or(0.95);
			Ok(Box::new(Normalize::new(peak)))
		}
		_ => Err(IoError::with_message(IoErrorKind::InvalidData, "unknown transform")),
	}
}

pub struct TransformChain {
	transforms: Vec<Box<dyn Transform>>,
}

impl TransformChain {
	pub fn new() -> Self {
		Self { transforms: Vec::new() }
	}

	pub fn add(&mut self, transform: Box<dyn Transform>) {
		self.transforms.push(transform);
	}

	pub fn is_empty(&self) -> bool {
		self.transforms.is_empty()
	}
}

impl Default for TransformChain {
	fn default() -> Self {
		Self::new()
	}
}

impl Transform for TransformChain {
	fn apply(&mut self, mut frame: crate::core::Frame) -> IoResult<crate::core::Frame> {
		for transform in &mut self.transforms {
			frame = transform.apply(frame)?;
		}
		Ok(frame)
	}

	fn name(&self) -> &'static str {
		"chain"
	}
}
