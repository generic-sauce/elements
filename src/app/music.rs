use crate::prelude::*;
use rodio::*;

pub enum MusicCommand {
	PlayMusic(MusicId),
}

struct Sample {
	channels: u16,
	sample_rate: u32,
	data: Vec<f32>,
}

lazy_static! {
	static ref PARTS: Vec<Sample> = load_samples();
}

fn load_samples() -> Vec<Sample> {
	let mut parts: Vec<Sample> = Vec::new();
	for music_id in MusicId::iter() {
		let file = File::open(res(music_id.filename())).unwrap();
		let source = Decoder::new(BufReader::new(file)).unwrap();
		let channels = source.channels();
		let sample_rate = source.sample_rate();
		let data = source.convert_samples().collect();
		let sample = Sample {
			channels,
			sample_rate,
			data,
		};
		parts.push(sample);
	}
	parts
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
				let sample = &PARTS[sound_id as usize];
				let sample_buffer = static_buffer::StaticSamplesBuffer::new(
					sample.channels,
					sample.sample_rate,
					&sample.data[..],
				);
				play_raw(&self.device, sample_buffer);
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
