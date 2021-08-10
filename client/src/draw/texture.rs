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
	Controls: "images/controls.png",
	Bush0: "images/bush0.png",
	BushFlowers0: "images/bush_flowers0.png",
	FloatingBush0: "images/floating_bush0.png",
	Grass0: "images/grass0.png",
	GrassStraws0: "images/grass_straws0.png",
	HangingBush0: "images/hanging_bush0.png",
	Mountains0: "images/mountains0.png",
	Mountains1: "images/mountains1.png",
	Stone0: "images/stone0.png",
	Stone1: "images/stone1.png",
	WideBush0: "images/wide_bush0.png",
);

pub trait IntoTextureIndex {
	fn into_texture_index(self) -> TextureIndex;
}

impl TextureId {
	pub fn texture_count() -> usize {
		TextureId::iter().count()
		+
		AnimationId::iter()
			.map(AnimationId::frame_count)
			.sum::<usize>()
	}
}

impl IntoTextureIndex for TextureIndex {
	fn into_texture_index(self) -> TextureIndex {
		self
	}
}

impl IntoTextureIndex for TextureId {
	fn into_texture_index(self) -> TextureIndex {
		self as TextureIndex
	}
}

impl IntoTextureIndex for Animation {
	fn into_texture_index(self) -> TextureIndex {
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
