pub trait AudioBackend {
	fn new() -> Self;
	fn tick(&mut self);
	fn queue_music(&mut self, music_id: SoundId);
	fn play_sound(&mut self, sound_id: SoundId, volume: f32);
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
