use crate::prelude::*;
use rodio::*;

pub enum MusicCommand {
	PlayMusic(MusicId),
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
	for music_id in MusicId::iter() {
		let file = File::open(res(music_id.filename())).unwrap();
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

fn get_sample_buffer(sound_id: MusicId) -> static_buffer::StaticSamplesBuffer<f32> {
	let sample = &SOUNDS[sound_id as usize];
	static_buffer::StaticSamplesBuffer::new(
		sample.channels,
		sample.sample_rate,
		&sample.data[..],
	)
}


pub struct Musician {
	receiver: Receiver<MusicCommand>,
	device: Device,
}


impl Musician {
	pub fn new(receiver: Receiver<MusicCommand>) -> Musician {
		Musician {
			receiver,
			device: default_output_device().unwrap(),
		}
	}

	pub fn run(&mut self) {
		for _ in TimedLoop::with_fps(10) {
			match self.receiver.try_recv() {
				Ok(c) => self.apply_command(c),
				Err(TryRecvError::Disconnected) => panic!("musician is disconnected!"),
				Err(TryRecvError::Empty) => {},
			}
		}
	}

	fn apply_command(&mut self, command: MusicCommand) {
		match command {
			MusicCommand::PlayMusic(sound_id) => {
				play_raw(&self.device, get_sample_buffer(sound_id));
			},
		}
	}
}

macro_rules! setup {
	($($id:ident : $file:expr),*$(,)?) => {
		#[derive(Copy, Clone, Debug, Eq, PartialEq)]
		#[repr(usize)]
		pub enum MusicId {
			$($id),*
		}

		impl MusicId {
			#[allow(unused)]
			pub fn iter() -> impl Iterator<Item=MusicId> {
				[$(MusicId::$id),*].iter().cloned()
			}

			#[allow(unused)]
			pub fn filename(self) -> &'static str {
				match self {
					$(
						MusicId::$id => $file,
					)*
				}
			}
		}

		use std::fmt::{Display, Formatter, Error};

		impl Display for MusicId {
			fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
				let string = match *self {
					$( MusicId::$id => std::stringify!($id), )*
				};
				write!(f, "{}", string)
			}
		}
	};
}

setup!(
	APart: "audio/a_part.wav",
);
