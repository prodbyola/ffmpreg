use crate::io::{IoError, IoResult, MediaRead, MediaWrite};
use std::io::{Read, Write};

pub struct StdinAdapter;

impl StdinAdapter {
	pub fn new() -> Self {
		Self
	}
}

impl MediaRead for StdinAdapter {
	fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
		std::io::stdin().read(buf).map_err(IoError::from)
	}
}

pub struct StdoutAdapter;

impl StdoutAdapter {
	pub fn new() -> Self {
		Self
	}
}

impl MediaWrite for StdoutAdapter {
	fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
		std::io::stdout().write(buf).map_err(IoError::from)
	}

	fn flush(&mut self) -> IoResult<()> {
		std::io::stdout().flush().map_err(IoError::from)
	}
}

pub enum StdioSource {
	Stdin(StdinAdapter),
	File(std::fs::File),
}

impl MediaRead for StdioSource {
	fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
		match self {
			StdioSource::Stdin(stdin) => stdin.read(buf),
			StdioSource::File(file) => {
				use std::io::Read;
				file.read(buf).map_err(IoError::from)
			}
		}
	}
}

pub enum StdioSink {
	Stdout(StdoutAdapter),
	File(std::fs::File),
}

impl MediaWrite for StdioSink {
	fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
		match self {
			StdioSink::Stdout(stdout) => stdout.write(buf),
			StdioSink::File(file) => {
				use std::io::Write;
				file.write(buf).map_err(IoError::from)
			}
		}
	}

	fn flush(&mut self) -> IoResult<()> {
		match self {
			StdioSink::Stdout(stdout) => stdout.flush(),
			StdioSink::File(file) => {
				use std::io::Write;
				file.flush().map_err(IoError::from)
			}
		}
	}
}
