use crate::{error, message};

pub fn get_extension(path: &str) -> message::Result<String> {
	std::path::Path::new(path)
		.extension()
		.and_then(|e| e.to_str())
		.map(|s| s.to_lowercase())
		.ok_or_else(|| error!("no file extension"))
}
