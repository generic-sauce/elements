use crate::prelude::*;
use rodio::*;

const SOUND_FPS: u32 = 10;
const MUSIC_DURATION: u32 = 16 * SOUND_FPS;

pub enum SoundCommand {
	PlayMusic(SoundId),
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

fn get_sample_buffer(sound_id: SoundId) -> static_buffer::StaticSamplesBuffer<f32> {
	let sample = &SOUNDS[sound_id as usize];
	static_buffer::StaticSamplesBuffer::new(
		sample.channels,
		sample.sample_rate,
		&sample.data[..],
	)
}


pub struct SoundManager {
	receiver: Receiver<SoundCommand>,
	device: Device,
	music_sink: Sink,
	current_music_id: Option<SoundId>,
	next_music_id: Option<SoundId>,
	music_counter: u32,
}


impl SoundManager {
	pub fn new(receiver: Receiver<SoundCommand>) -> SoundManager {
		let device = default_output_device().unwrap();
		let music_sink = Sink::new(&device);
		SoundManager {
			receiver,
			device,
			music_sink,
			current_music_id: None,
			next_music_id: None,
			music_counter: 0,
		}
	}

	pub fn run(&mut self) {
		for _ in TimedLoop::with_fps(SOUND_FPS) {
			match self.receiver.try_recv() {
				Ok(c) => self.apply_command(c),
				Err(TryRecvError::Disconnected) => panic!("sound manager is disconnected!"),
				Err(TryRecvError::Empty) => {},
			}

			self.check_start_music();
			self.check_restart_music();
		}
	}

	fn check_start_music(&mut self) {
		if self.current_music_id.is_none() {
			if let Some(next_music_id) = self.next_music_id {
				self.music_sink.append(get_sample_buffer(next_music_id));
				self.current_music_id = Some(next_music_id);
				self.music_counter = 0;
			}
		}
	}

	fn check_restart_music(&mut self) {
		if let Some(next_music_id) = self.next_music_id {
			if self.music_counter == MUSIC_DURATION {
				if self.music_sink.len() <= 1 {
					self.music_sink.append(get_sample_buffer(next_music_id));
					self.current_music_id = Some(next_music_id);
				}
				self.music_counter = 0;
			}
			self.music_counter += 1;
		}
	}

	fn apply_command(&mut self, command: SoundCommand) {
		match command {
			SoundCommand::PlayMusic(music_id) => {
				println!("next music: {}", music_id);
				self.next_music_id = Some(music_id);
			},
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
);
