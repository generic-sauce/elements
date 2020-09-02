use crate::prelude::*;
use rodio::*;

const START_MUSIC_OFFSET: Duration = Duration::from_micros(200);
const NUM_PARTS: usize = 4;

pub enum SoundCommand {
	PlayMusic(SoundId),
	PlaySound(SoundId),
}

struct Sound {
	channels: u16,
	sample_rate: u32,
	data: Vec<f32>,
}

lazy_static! {
	static ref SOUNDS: Vec<Sound> = load_samples();
}

fn load_samples() -> Vec<Sound> {
	let mut parts: Vec<Sound> = Vec::new();
	for sound_id in SoundId::iter() {
		let file = File::open(res(sound_id.filename())).unwrap();
		let source = Decoder::new(BufReader::new(file)).unwrap();
		let channels = source.channels();
		let sample_rate = source.sample_rate();
		let data = source.convert_samples().collect();
		let sample = Sound {
			channels,
			sample_rate,
			data,
		};
		parts.push(sample);
	}
	parts
}

fn get_part_sample_buffer(sound_id: SoundId, part: u8) -> static_buffer::StaticSamplesBuffer<f32> {
	let part = part as usize;
	let sample = &SOUNDS[sound_id as usize];
	let part_size = sample.data.len() / NUM_PARTS;

	static_buffer::StaticSamplesBuffer::new(
		sample.channels,
		sample.sample_rate,
		&sample.data[part*part_size..(part+1)*part_size]
	)
}

fn get_sample_buffer(sound_id: SoundId) -> static_buffer::StaticSamplesBuffer<f32> {
	let sample = &SOUNDS[sound_id as usize];
	static_buffer::StaticSamplesBuffer::new(
		sample.channels,
		sample.sample_rate,
		&sample.data[..],
	)
}

pub struct SoundManager {
	device: Device,
	music_sink: Sink,
	current_music_id: Option<SoundId>,
	next_music_id: Option<SoundId>,
	next_part: u8,
	next_music_refresh_time: Instant,
}


impl SoundManager {
	pub fn new() -> SoundManager {
		let device = default_output_device().unwrap();
		let music_sink = Sink::new(&device);
		SoundManager {
			device,
			music_sink,
			current_music_id: None,
			next_music_id: None,
			next_part: 0,
			next_music_refresh_time: Instant::now(),
		}
	}

	pub fn tick(&mut self) {
		self.check_start_music();
		self.check_restart_music();
	}

	fn play_music(&mut self, music_id: SoundId) {
		let sample = get_part_sample_buffer(music_id, self.next_part);
		let sample_duration = sample.total_duration().unwrap();
		self.music_sink.append(sample);
		self.current_music_id = Some(music_id);
		self.next_music_refresh_time += sample_duration;
		self.next_part = (self.next_part + 1) % NUM_PARTS as u8;
	}

	fn check_start_music(&mut self) {
		if self.current_music_id.is_none() {
			if let Some(next_music_id) = self.next_music_id {
				self.next_music_refresh_time = Instant::now();
				self.play_music(next_music_id);
			}
		}
	}

	fn check_restart_music(&mut self) {
		let should_refresh = self.next_music_refresh_time.saturating_duration_since(Instant::now()) < START_MUSIC_OFFSET;
		if should_refresh {
			if let Some(next_music_id) = self.next_music_id {
				assert_eq!(self.music_sink.len(), 1);
				self.play_music(next_music_id);
			}
		}
	}

	pub fn apply_command(&mut self, command: SoundCommand) {
		match command {
			SoundCommand::PlayMusic(music_id) => {
				self.next_music_id = Some(music_id);
			},
			SoundCommand::PlaySound(sound_id) => {
				play_raw(&self.device, get_sample_buffer(sound_id));
			}
		}
	}
}

macro_rules! setup {
	($($id:ident : $file:expr),*$(,)?) => {
		#[derive(Copy, Clone, Debug, Eq, PartialEq)]
		#[repr(usize)]
		pub enum SoundId {
			$($id),*
		}

		impl SoundId {
			#[allow(unused)]
			pub fn iter() -> impl Iterator<Item=SoundId> {
				[$(SoundId::$id),*].iter().cloned()
			}

			#[allow(unused)]
			pub fn filename(self) -> &'static str {
				match self {
					$(
						SoundId::$id => $file,
					)*
				}
			}
		}

		use std::fmt::{Display, Formatter, Error};

		impl Display for SoundId {
			fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
				let string = match *self {
					$( SoundId::$id => std::stringify!($id), )*
				};
				write!(f, "{}", string)
			}
		}
	};
}

setup!(
	APart: "audio/a_part.wav",
	BPart: "audio/b_part.wav",
	CPart: "audio/c_part.wav",
	DPart: "audio/d_part.wav",
	Whiz: "audio/whiz.wav",
);
