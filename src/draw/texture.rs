use crate::prelude::*;

macro_rules! setup {
	($($id:ident : $resource:expr),*$(,)?) => {

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

			// TODO use in the web client
			#[allow(unused)]
			pub fn filepath_relative(self) -> &'static str {
				match self {
					$(
						TextureId::$id => $resource,
					)*
				}
			}

			#[allow(unused)]
			#[cfg(feature = "server")] /* or native-client */
			pub fn filepath(self) -> String {
				res(self.filepath_relative())
			}
		}
	}
}

setup!(
	Unknown: "images/checkerboard.png",
	White: "images/white.png",
	White2: "images/white.png",
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

impl IntoTextureIndex for TextureId {
	fn into_texture_index(self) -> usize {
		self as usize
	}
}

impl IntoTextureIndex for Animation {
	fn into_texture_index(self) -> usize {
		let texture_offset = TextureId::iter().count();
		let animation_offset = AnimationId::iter()
			.enumerate()
			.filter(|(index, _)| *index < self.animation_id as usize)
			.map(|(_, id)| id)
			.fold(0, |acc, id| acc + AnimationId::frame_count(id));

		texture_offset + animation_offset + self.texture_index()
	}
}
