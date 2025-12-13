use ffmpreg::io::{MediaWrite, StdWriteAdapter, WritePrimitives};

#[test]
fn test_vec_write() {
	let mut writer: Vec<u8> = Vec::new();

	let n = writer.write(&[1, 2, 3]).unwrap();
	assert_eq!(n, 3);
	assert_eq!(&writer, &[1, 2, 3]);

	let n = writer.write(&[4, 5]).unwrap();
	assert_eq!(n, 2);
	assert_eq!(&writer, &[1, 2, 3, 4, 5]);
}

#[test]
fn test_write_all() {
	let mut writer: Vec<u8> = Vec::new();

	writer.write_all(&[1, 2, 3, 4]).unwrap();
	assert_eq!(&writer, &[1, 2, 3, 4]);
}

#[test]
fn test_write_primitives_be() {
	let mut writer: Vec<u8> = Vec::new();

	writer.write_u16_be(0x0102).unwrap();
	writer.write_u32_be(0x03040506).unwrap();
	writer.write_u64_be(0x0708090A0B0C0D0E).unwrap();

	assert_eq!(
		&writer,
		&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E]
	);
}

#[test]
fn test_write_primitives_le() {
	let mut writer: Vec<u8> = Vec::new();

	writer.write_u16_le(0x0201).unwrap();
	writer.write_u32_le(0x06050403).unwrap();

	assert_eq!(&writer, &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
}

#[test]
fn test_std_write_adapter() {
	use std::io::Cursor;

	let cursor = Cursor::new(Vec::new());
	let mut writer = StdWriteAdapter::new(cursor);

	writer.write_u32_be(0xDEADBEEF).unwrap();
	writer.flush().unwrap();

	let inner = writer.into_inner();
	assert_eq!(inner.into_inner(), vec![0xDE, 0xAD, 0xBE, 0xEF]);
}

#[test]
fn test_write_signed_primitives() {
	let mut writer: Vec<u8> = Vec::new();

	writer.write_i8(-1).unwrap();
	writer.write_i16_be(-256).unwrap();
	writer.write_i32_le(1).unwrap();

	assert_eq!(writer.len(), 1 + 2 + 4);
	assert_eq!(writer[0], 0xFF);
}

#[test]
fn test_write_float_primitives() {
	let mut writer: Vec<u8> = Vec::new();

	writer.write_f32_be(1.0).unwrap();
	assert_eq!(writer.len(), 4);
	assert_eq!(&writer, &1.0_f32.to_be_bytes());

	writer.clear();
	writer.write_f64_le(2.0).unwrap();
	assert_eq!(writer.len(), 8);
	assert_eq!(&writer, &2.0_f64.to_le_bytes());
}

struct PartialWriter {
	data: Vec<u8>,
	chunk_size: usize,
}

impl PartialWriter {
	fn new(chunk_size: usize) -> Self {
		Self { data: Vec::new(), chunk_size }
	}
}

impl MediaWrite for PartialWriter {
	fn write(&mut self, buf: &[u8]) -> ffmpreg::io::IoResult<usize> {
		let to_write = core::cmp::min(buf.len(), self.chunk_size);
		self.data.extend_from_slice(&buf[..to_write]);
		Ok(to_write)
	}

	fn flush(&mut self) -> ffmpreg::io::IoResult<()> {
		Ok(())
	}
}

#[test]
fn test_partial_writes_with_write_all() {
	let mut writer = PartialWriter::new(2);

	writer.write_all(&[1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
	assert_eq!(&writer.data, &[1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_partial_writes_primitives() {
	let mut writer = PartialWriter::new(1);
	writer.write_u32_be(0x01020304).unwrap();
	assert_eq!(&writer.data, &[0x01, 0x02, 0x03, 0x04]);
}
