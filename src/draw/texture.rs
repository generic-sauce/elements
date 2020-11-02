use crate::prelude::*;

macro_rules! setup {
	($($id:ident : $resource:expr),*) => {

		#[derive(Copy, Clone, Debug)]
		#[repr(usize)]
		#[allow(unused)]
		pub enum TextureId {
			$($id),*
		}

		impl TextureId {
			#[allow(unused)]
			pub fn iter() -> impl Iterator<Item=TextureId> {
				[$(TextureId::$id),*].iter().cloned()
			}

			#[allow(unused)]
			pub fn filepath(self) -> String {
				match self {
					$(
						TextureId::$id => res($resource),
					)*
				}
			}
		}
	}
}

setup!(
	Unknown: "images/checkerboard.png",
	White: "images/white.png"
);

impl TextureId {
	pub fn texture_count() -> usize {
		TextureId::iter().count()
		+
		AnimationId::iter()
			.map(AnimationId::frame_count)
			.sum::<usize>()
	}
}
