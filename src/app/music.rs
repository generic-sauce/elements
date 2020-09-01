use crate::prelude::*;
use rodio::Source;
use std::io::Read;

pub enum MusicCommand {
	PlayMusic(MusicId),
}

lazy_static! {
	static ref PARTS: Vec<Vec<u8>> = load_samples();
}

fn load_samples() -> Vec<Vec<u8>> {
	let mut parts: Vec<Vec<u8>> = Vec::new();
	for music_id in MusicId::iter() {
		let mut file = File::open(res(music_id.filename())).unwrap();
		let mut vec = Vec::new();
		file.read_to_end(&mut vec).unwrap();
		parts.push(vec);
	}
	parts
}

pub struct Musician {
	receiver: Receiver<MusicCommand>,
	device: rodio::Device,
}


impl Musician {
	pub fn new(receiver: Receiver<MusicCommand>) -> Musician {
		Musician {
			receiver,
			device: rodio::default_output_device().unwrap(),
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
				let file = std::io::Cursor::new(&PARTS[sound_id as usize][..]);
				let source = rodio::Decoder::new(file).unwrap();
				rodio::play_raw(&self.device, source.convert_samples());
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
