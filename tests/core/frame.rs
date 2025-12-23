use ffmpreg::core::{Frame, FrameAudio, FrameVideo, Timebase, VideoFormat};

#[test]
fn test_frame_audio_creation() {
	let data = vec![0u8; 1024];
	let timebase = Timebase::new(1, 44100);
	let audio = FrameAudio::new(data.clone(), 44100, 2);
	let frame = Frame::new_audio(audio, timebase, 0);

	assert_eq!(frame.size(), 1024);
	assert!(frame.is_audio_frame());
	assert!(!frame.is_video_frame());
}

#[test]
fn test_frame_video_creation() {
	let data = vec![0u8; 1024];
	let timebase = Timebase::new(1, 30);
	let video = FrameVideo::new(data.clone(), 640, 480, VideoFormat::RGB24);
	let frame = Frame::new_video(video, timebase, 0);

	assert_eq!(frame.size(), 1024);
	assert!(!frame.is_audio_frame());
	assert!(frame.is_video_frame());
}

#[test]
fn test_frame_with_pts() {
	let data = vec![0u8; 512];
	let timebase = Timebase::new(1, 48000);
	let audio = FrameAudio::new(data, 48000, 1);
	let frame = Frame::new_audio(audio, timebase, 0).with_pts(1024);

	assert_eq!(frame.pts, 1024);
}

#[test]
fn test_frame_empty() {
	let data = vec![];
	let timebase = Timebase::new(1, 44100);
	let audio = FrameAudio::new(data, 44100, 1);
	let frame = Frame::new_audio(audio, timebase, 0);

	assert!(frame.is_empty());
	assert_eq!(frame.size(), 0);
}

#[test]
fn test_frame_audio_mut() {
	let data = vec![1, 2, 3, 4];
	let timebase = Timebase::new(1, 44100);
	let audio = FrameAudio::new(data, 44100, 1);
	let mut frame = Frame::new_audio(audio, timebase, 0);

	if let Some(audio_frame) = frame.audio_mut() {
		audio_frame.data[0] = 255;
	}

	if let Some(audio_frame) = frame.audio() {
		assert_eq!(audio_frame.data[0], 255);
	}
}

#[test]
fn test_frame_video_mut() {
	let data = vec![1, 2, 3, 4];
	let timebase = Timebase::new(1, 30);
	let video = FrameVideo::new(data, 2, 2, VideoFormat::GRAY8);
	let mut frame = Frame::new_video(video, timebase, 0);

	if let Some(video_frame) = frame.video_mut() {
		video_frame.data[0] = 255;
	}

	if let Some(video_frame) = frame.video() {
		assert_eq!(video_frame.data[0], 255);
	}
}

#[test]
fn test_frame_audio_clone() {
	let data = vec![1, 2, 3, 4];
	let timebase = Timebase::new(1, 44100);
	let audio = FrameAudio::new(data, 44100, 1);
	let frame = Frame::new_audio(audio, timebase, 0).with_pts(100);
	let cloned = frame.clone();

	assert_eq!(cloned.pts, frame.pts);
	assert_eq!(cloned.size(), frame.size());
	assert!(cloned.is_audio_frame());
}

#[test]
fn test_frame_kind() {
	let data = vec![1, 2, 3, 4];
	let timebase = Timebase::new(1, 44100);
	let audio = FrameAudio::new(data.clone(), 44100, 1);
	let frame_audio = Frame::new_audio(audio, timebase.clone(), 0);

	match frame_audio.kind() {
		ffmpreg::core::FrameKind::Audio => assert!(true),
		ffmpreg::core::FrameKind::Video => panic!("Expected Audio frame"),
	}

	let video = FrameVideo::new(data, 2, 2, VideoFormat::GRAY8);
	let frame_video = Frame::new_video(video, timebase, 0);

	match frame_video.kind() {
		ffmpreg::core::FrameKind::Video => assert!(true),
		ffmpreg::core::FrameKind::Audio => panic!("Expected Video frame"),
	}
}
