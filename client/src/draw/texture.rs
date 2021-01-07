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

			#[allow(unused)]
			pub fn filepath(self) -> String {
				res(match self {
					$(
						TextureId::$id => $resource,
					)*
				})
			}
		}
	}
}

setup!(
	Unknown: "images/checkerboard.png",
	Icon: "icon01.png",
	White: "images/white.png",
	Trophy: "images/trophy/trophy.png",
	SkyBackground: "images/background/sky_background.png",
	Globe: "images/icons/globe.png",
	Gamepad: "images/icons/gamepad.png",
);

pub type TextureIndex = usize;

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

pub fn texture_filenames_iter() -> impl Iterator<Item=String> {
	let textures = TextureId::iter()
		.map(TextureId::filepath);

	let animation_textures = AnimationId::iter()
		.flat_map(AnimationId::filepaths_iter);

	textures.chain(animation_textures)
}
