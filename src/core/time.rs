#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timebase {
	pub num: u32,
	pub den: u32,
}

impl Timebase {
	pub fn new(num: u32, den: u32) -> Self {
		assert!(num > 0 && den > 0, "timebase numerator and denominator must be positive");
		Self { num, den }
	}

	pub fn to_seconds(&self, pts: i64) -> f64 {
		(pts as f64) * (self.num as f64) / (self.den as f64)
	}

	pub fn from_seconds(&self, seconds: f64) -> i64 {
		((seconds * self.den as f64) / self.num as f64) as i64
	}
}
