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
	($($id:ident : $dir:expr, $frame_count:expr, $interval:expr),*$(,)?) => {
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
			pub fn dir(self) -> &'static str {
				match self {
					$(
						AnimationId::$id => $dir,
					)*
				}
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
	BluePlayerIdle: "images/player_blue/player_idle", 5, 8,
	BluePlayerRun: "images/player_blue/player_run", 6, 4,
	BluePlayerJump: "images/player_blue/player_jump", 2, 4,
	BluePlayerFall: "images/player_blue/player_fall", 2, 4,
	BluePlayerFallSlow: "images/player_blue/player_fall_slow", 2, 4,
	RedPlayerIdle: "images/player_red/player_idle", 5, 8,
	RedPlayerRun: "images/player_red/player_run", 4, 4,
	RedPlayerJump: "images/player_red/player_jump", 2, 4,
	RedPlayerFall: "images/player_red/player_fall", 2, 4,
	RedPlayerFallSlow: "images/player_red/player_fall_slow", 2, 4
);
