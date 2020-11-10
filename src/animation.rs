use crate::prelude::*;

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
		self.index = (self.index + 1) % (self.animation_id.frame_count() * self.animation_id.interval());
	}

	pub fn texture_index(&self) -> usize {
		self.index / self.animation_id.interval()
	}
}

macro_rules! setup {
	($($id:ident : $parent_dir:expr, $name:expr, $extension:expr, $frame_count:expr, $interval:expr),*$(,)?) => {
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
		}
	};
}

setup!(
	BluePlayerIdle: "images/player_blue", "player_idle", "png", 5, 9,
	BluePlayerRun: "images/player_blue", "player_run", "png", 6, 6,
	BluePlayerJump: "images/player_blue", "player_jump", "png", 2, 6,
	BluePlayerFall: "images/player_blue", "player_fall", "png", 2, 6,
	BluePlayerFallSlow: "images/player_blue", "player_fall_slow", "png", 2, 6,
	RedPlayerIdle: "images/player_red", "player_idle", "png", 5, 9,
	RedPlayerRun: "images/player_red", "player_run", "png", 4, 6,
	RedPlayerJump: "images/player_red", "player_jump", "png", 2, 6,
	RedPlayerFall: "images/player_red", "player_fall", "png", 2, 6,
	RedPlayerFallSlow: "images/player_red", "player_fall_slow", "png", 2, 6
);
