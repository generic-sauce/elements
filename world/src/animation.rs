use crate::prelude::*;

pub type TextureIndex = usize;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum WrapMode {
	Reset,
	Reverse,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Animation {
	pub animation_id: AnimationId,
	pub index: usize,
}

impl Animation {
	pub fn new(animation_id: AnimationId) -> Animation {
		Animation {
			animation_id,
			index: 0,
		}
	}

	pub fn tick(&mut self) {
		self.index = self.index + 1
	}

	pub fn texture_index(&self) -> TextureIndex {
		let count = self.animation_id.frame_count() as i64;
		let index = self.index / self.animation_id.interval();
		let index = index as i64;
		(count - i64::abs(index - count)) as usize;

		let i = match self.animation_id.wrap_mode() {
			WrapMode::Reset => index % count, // sawtooth wave
			WrapMode::Reverse => i64::abs(index % (2*count-2) - count + 1) // triangle wave
		} as usize;
		i
	}
}

macro_rules! setup {
	($($id:ident : $parent_dir:expr, $name:expr, $extension:expr, $frame_count:expr, $interval:expr, $wrap_mode:expr),*$(,)?) => {
		#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
		#[repr(usize)]
		pub enum AnimationId {
			$($id),*
		}

		use std::fmt::{Display, Formatter, Error};

		impl Display for AnimationId {
			fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
				let string = match *self {
					$( AnimationId::$id => std::stringify!($id), )*
				};
				write!(f, "{}", string)
			}
		}

		impl AnimationId {
			#[allow(unused)]
			pub fn iter() -> impl Iterator<Item=AnimationId> {
				[$(AnimationId::$id),*].iter().cloned()
			}

			#[allow(unused)]
			pub fn iter_animations() -> impl Iterator<Item=Animation> {
				Self::iter()
					.flat_map(|id| (0..id.frame_count())
						.map(move |index| Animation {
							animation_id: id,
							index,
						})
					)
			}

			#[allow(unused)]
			pub fn name(self) -> &'static str {
				match self {
					$(
						AnimationId::$id => $name,
					)*
				}
			}

			#[allow(unused)]
			pub fn extension(self) -> &'static str {
				match self {
					$(
						AnimationId::$id => $extension,
					)*
				}
			}

			#[allow(unused)]
			pub fn dir(self) -> String {
				match self {
					$(
						AnimationId::$id => format!("{}/{}", $parent_dir, $name),
					)*
				}
			}

			pub fn filepaths_iter(self) -> impl Iterator<Item=String> {
				let id = self;
				let dir = AnimationId::dir(id);
				let name = AnimationId::name(id);
				let extension = AnimationId::extension(id);
				(0..AnimationId::frame_count(id))
					.map(move |i| {
						let filepath = format!("{}/{}{}.{}", dir, name, i + 1, extension);
						let filepath = res(filepath.as_str());
						filepath
					})
			}

			pub fn frame_count(self) -> usize {
				match self {
					$(
						AnimationId::$id => $frame_count,
					)*
				}
			}

			pub fn interval(self) -> usize {
				match self {
					$(
						AnimationId::$id => $interval,
					)*
				}
			}

			pub fn wrap_mode(self) -> WrapMode {
				match self {
					$(
						AnimationId::$id => $wrap_mode,
					)*
				}
			}
		}
	};
}

setup!(
	BluePlayerIdle: "images/player_blue", "player_idle", "png", 5, 9, WrapMode::Reset,
	BluePlayerRun: "images/player_blue", "player_run", "png", 6, 6, WrapMode::Reset,
	BluePlayerJump: "images/player_blue", "player_jump", "png", 2, 6, WrapMode::Reset,
	BluePlayerFall: "images/player_blue", "player_fall", "png", 2, 6, WrapMode::Reset,
	BluePlayerFallSlow: "images/player_blue", "player_fall_slow", "png", 2, 6, WrapMode::Reset,
	RedPlayerIdle: "images/player_red", "player_idle", "png", 5, 9, WrapMode::Reset,
	RedPlayerRun: "images/player_red", "player_run", "png", 4, 6, WrapMode::Reset,
	RedPlayerJump: "images/player_red", "player_jump", "png", 2, 6, WrapMode::Reset,
	RedPlayerFall: "images/player_red", "player_fall", "png", 2, 6, WrapMode::Reset,
	RedPlayerFallSlow: "images/player_red", "player_fall_slow", "png", 6, 6, WrapMode::Reset,
	Bird: "images", "bird", "png", 6, 10, WrapMode::Reverse,
);
