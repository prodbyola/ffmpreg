use ffmpreg::io::{BufferedReader, IoErrorKind, MediaRead, ReadPrimitives, StdReadAdapter};

#[test]
fn test_slice_read() {
	let data: &[u8] = &[1, 2, 3, 4, 5];
	let mut reader = data;
	let mut buf = [0u8; 3];

	let n = reader.read(&mut buf).unwrap();
	assert_eq!(n, 3);
	assert_eq!(&buf, &[1, 2, 3]);

	let n = reader.read(&mut buf).unwrap();
	assert_eq!(n, 2);
	assert_eq!(&buf[..2], &[4, 5]);
}

#[test]
fn test_slice_read_eof() {
	let data: &[u8] = &[1, 2];
	let mut reader = data;
	let mut buf = [0u8; 4];

	let n = reader.read(&mut buf).unwrap();
	assert_eq!(n, 2);

	let n = reader.read(&mut buf).unwrap();
	assert_eq!(n, 0);
}

#[test]
fn test_read_exact_success() {
	let data: &[u8] = &[0xDE, 0xAD, 0xBE, 0xEF];
	let mut reader = data;
	let mut buf = [0u8; 4];

	reader.read_exact(&mut buf).unwrap();
	assert_eq!(&buf, &[0xDE, 0xAD, 0xBE, 0xEF]);
}

#[test]
fn test_read_exact_eof() {
	let data: &[u8] = &[0xDE, 0xAD];
	let mut reader = data;
	let mut buf = [0u8; 4];

	let result = reader.read_exact(&mut buf);
	assert!(result.is_err());
	assert!(matches!(result.unwrap_err().kind(), IoErrorKind::UnexpectedEof));
}

#[test]
fn test_read_primitives_be() {
	let data: &[u8] =
		&[0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03];
	let mut reader = data;

	assert_eq!(reader.read_u16_be().unwrap(), 1);
	assert_eq!(reader.read_u32_be().unwrap(), 2);
	assert_eq!(reader.read_u64_be().unwrap(), 3);
}

#[test]
fn test_read_primitives_le() {
	let data: &[u8] =
		&[0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
	let mut reader = data;

	assert_eq!(reader.read_u16_le().unwrap(), 1);
	assert_eq!(reader.read_u32_le().unwrap(), 2);
	assert_eq!(reader.read_u64_le().unwrap(), 3);
}

#[test]
fn test_buffered_reader_small_reads() {
	let data: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
	let inner = data;
	let mut reader = BufferedReader::<_, 4>::new(inner);

	let mut buf = [0u8; 2];
	assert_eq!(reader.read(&mut buf).unwrap(), 2);
	assert_eq!(&buf, &[1, 2]);

	assert_eq!(reader.read(&mut buf).unwrap(), 2);
	assert_eq!(&buf, &[3, 4]);

	assert_eq!(reader.read(&mut buf).unwrap(), 2);
	assert_eq!(&buf, &[5, 6]);
}

#[test]
fn test_buffered_reader_large_read_bypass() {
	let data: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8];
	let inner = data;
	let mut reader = BufferedReader::<_, 4>::new(inner);

	let mut buf = [0u8; 8];
	let n = reader.read(&mut buf).unwrap();
	assert_eq!(n, 8);
	assert_eq!(&buf, &[1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_buffered_reader_partial_buffer() {
	let data: &[u8] = &[1, 2, 3];
	let inner = data;
	let mut reader = BufferedReader::<_, 8>::new(inner);

	let mut buf = [0u8; 2];
	assert_eq!(reader.read(&mut buf).unwrap(), 2);
	assert_eq!(&buf, &[1, 2]);

	assert_eq!(reader.read(&mut buf).unwrap(), 1);
	assert_eq!(buf[0], 3);

	assert_eq!(reader.read(&mut buf).unwrap(), 0);
}

#[test]
fn test_std_read_adapter() {
	use std::io::Cursor;

	let cursor = Cursor::new(vec![0xCA, 0xFE, 0xBA, 0xBE]);
	let mut reader = StdReadAdapter::new(cursor);

	assert_eq!(reader.read_u32_be().unwrap(), 0xCAFEBABE);
}

struct PartialReader {
	data: Vec<u8>,
	pos: usize,
	chunk_size: usize,
}

impl PartialReader {
	fn new(data: Vec<u8>, chunk_size: usize) -> Self {
		Self { data, pos: 0, chunk_size }
	}
}

impl MediaRead for PartialReader {
	fn read(&mut self, buf: &mut [u8]) -> ffmpreg::io::IoResult<usize> {
		let remaining = self.data.len() - self.pos;
		if remaining == 0 {
			return Ok(0);
		}
		let to_read = core::cmp::min(core::cmp::min(remaining, buf.len()), self.chunk_size);
		buf[..to_read].copy_from_slice(&self.data[self.pos..self.pos + to_read]);
		self.pos += to_read;
		Ok(to_read)
	}
}

#[test]
fn test_partial_reads_with_read_exact() {
	let mut reader = PartialReader::new(vec![1, 2, 3, 4, 5, 6, 7, 8], 2);
	let mut buf = [0u8; 8];

	reader.read_exact(&mut buf).unwrap();
	assert_eq!(&buf, &[1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_partial_reads_primitives() {
	let mut reader = PartialReader::new(vec![0x00, 0x00, 0x00, 0x01], 1);
	assert_eq!(reader.read_u32_be().unwrap(), 1);
}
