use ffmpreg::io::{MediaSeek, SeekFrom, StdSeekAdapter};

#[test]
fn test_seek_from_conversion() {
	let start = SeekFrom::Start(100);
	let std_start: std::io::SeekFrom = start.into();
	assert!(matches!(std_start, std::io::SeekFrom::Start(100)));

	let end = SeekFrom::End(-50);
	let std_end: std::io::SeekFrom = end.into();
	assert!(matches!(std_end, std::io::SeekFrom::End(-50)));

	let current = SeekFrom::Current(25);
	let std_current: std::io::SeekFrom = current.into();
	assert!(matches!(std_current, std::io::SeekFrom::Current(25)));
}

#[test]
fn test_std_seek_adapter() {
	use std::io::Cursor;

	let cursor = Cursor::new(vec![1, 2, 3, 4, 5, 6, 7, 8]);
	let mut seeker = StdSeekAdapter::new(cursor);

	assert_eq!(seeker.stream_position().unwrap(), 0);

	seeker.seek(SeekFrom::Start(4)).unwrap();
	assert_eq!(seeker.stream_position().unwrap(), 4);

	seeker.seek(SeekFrom::Current(2)).unwrap();
	assert_eq!(seeker.stream_position().unwrap(), 6);

	seeker.seek(SeekFrom::End(-2)).unwrap();
	assert_eq!(seeker.stream_position().unwrap(), 6);
}

#[test]
fn test_stream_len() {
	use std::io::Cursor;

	let cursor = Cursor::new(vec![1, 2, 3, 4, 5]);
	let mut seeker = StdSeekAdapter::new(cursor);

	seeker.seek(SeekFrom::Start(2)).unwrap();
	assert_eq!(seeker.stream_len().unwrap(), 5);
	assert_eq!(seeker.stream_position().unwrap(), 2);
}

#[test]
fn test_rewind() {
	use std::io::Cursor;

	let cursor = Cursor::new(vec![1, 2, 3, 4, 5]);
	let mut seeker = StdSeekAdapter::new(cursor);

	seeker.seek(SeekFrom::Start(3)).unwrap();
	assert_eq!(seeker.stream_position().unwrap(), 3);

	seeker.rewind().unwrap();
	assert_eq!(seeker.stream_position().unwrap(), 0);
}

#[test]
fn test_seekable_reader() {
	use ffmpreg::io::{IoError, IoResult, MediaRead, ReadPrimitives};
	use std::cell::RefCell;
	use std::io::Cursor;
	use std::rc::Rc;

	struct SharedCursor {
		inner: Rc<RefCell<Cursor<Vec<u8>>>>,
	}

	impl MediaRead for SharedCursor {
		fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
			use std::io::Read;
			self.inner.borrow_mut().read(buf).map_err(IoError::from)
		}
	}

	impl MediaSeek for SharedCursor {
		fn seek(&mut self, pos: SeekFrom) -> IoResult<u64> {
			use std::io::Seek;
			self.inner.borrow_mut().seek(pos.into()).map_err(IoError::from)
		}
	}

	let data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
	let cursor = Rc::new(RefCell::new(Cursor::new(data)));
	let mut seekable = SharedCursor { inner: cursor };

	let mut buf = [0u8; 2];
	seekable.read(&mut buf).unwrap();
	assert_eq!(&buf, &[0x01, 0x02]);

	seekable.seek(SeekFrom::Start(4)).unwrap();

	let val = seekable.read_u16_be().unwrap();
	assert_eq!(val, 0x0506);
}

#[test]
fn test_seekable_writer() {
	use ffmpreg::io::{IoError, IoResult, MediaWrite, WritePrimitives};
	use std::cell::RefCell;
	use std::io::Cursor;
	use std::rc::Rc;

	struct SharedCursor {
		inner: Rc<RefCell<Cursor<Vec<u8>>>>,
	}

	impl MediaWrite for SharedCursor {
		fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
			use std::io::Write;
			self.inner.borrow_mut().write(buf).map_err(IoError::from)
		}

		fn flush(&mut self) -> IoResult<()> {
			use std::io::Write;
			self.inner.borrow_mut().flush().map_err(IoError::from)
		}
	}

	impl MediaSeek for SharedCursor {
		fn seek(&mut self, pos: SeekFrom) -> IoResult<u64> {
			use std::io::Seek;
			self.inner.borrow_mut().seek(pos.into()).map_err(IoError::from)
		}
	}

	let cursor = Rc::new(RefCell::new(Cursor::new(vec![0u8; 8])));
	let mut seekable = SharedCursor { inner: cursor.clone() };

	seekable.write_u32_be(0xDEADBEEF).unwrap();
	seekable.seek(SeekFrom::Start(0)).unwrap();
	seekable.write_u16_be(0xCAFE).unwrap();
	seekable.flush().unwrap();

	let inner = cursor.borrow().clone().into_inner();
	assert_eq!(&inner[0..4], &[0xCA, 0xFE, 0xBE, 0xEF]);
}
