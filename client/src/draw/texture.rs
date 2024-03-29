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

pub trait IntoTextureId {
	fn to_texture_id(self) -> TextureId;
}

impl IntoTextureId for DetailType {
	fn to_texture_id(self) -> TextureId {
		match self {
			DetailType::Bush0 => TextureId::Bush0,
			DetailType::BushFlowers0 => TextureId::BushFlowers0,
			DetailType::FloatingBush0 => TextureId::FloatingBush0,
			DetailType::GrassStraws0 => TextureId::GrassStraws0,
			DetailType::HangingBush0 => TextureId::HangingBush0,
			DetailType::Mountains0 => TextureId::Mountains0,
			DetailType::Mountains1 => TextureId::Mountains1,
			DetailType::Stone0 => TextureId::Stone0,
			DetailType::Stone1 => TextureId::Stone1,
			DetailType::WideBush0 => TextureId::WideBush0,
		}
	}
}

pub trait IntoTextureOrigin {
	fn to_origin(self) -> TileVec;
}

impl IntoTextureOrigin for DetailType {
	fn to_origin(self) -> TileVec {
		match self {
			DetailType::Bush0 => TileVec::new(6, 0),
			DetailType::BushFlowers0 => TileVec::new(4, 0),
			DetailType::FloatingBush0 => TileVec::new(4, 4),
			DetailType::GrassStraws0 => TileVec::new(4, 2),
			DetailType::HangingBush0 => TileVec::new(8, 6),
			DetailType::Mountains0 => TileVec::new(0, 0),
			DetailType::Mountains1 => TileVec::new(80, 0),
			DetailType::Stone0 => TileVec::new(2, 0),
			DetailType::Stone1 => TileVec::new(2, 0),
			DetailType::WideBush0 => TileVec::new(8, 0),
		}
	}
}

pub trait IntoTextureSize {
	fn to_size(self) -> TileVec;
}

impl IntoTextureSize for DetailType {
	fn to_size(self) -> TileVec {
		match self {
			DetailType::Bush0 => TileVec::new(8, 8),
			DetailType::BushFlowers0 => TileVec::new(8, 8),
			DetailType::FloatingBush0 => TileVec::new(8, 8),
			DetailType::GrassStraws0 => TileVec::new(8, 8),
			DetailType::HangingBush0 => TileVec::new(16, 8),
			DetailType::Mountains0 => TileVec::new(128, 64),
			DetailType::Mountains1 => TileVec::new(80, 80),
			DetailType::Stone0 => TileVec::new(4, 4),
			DetailType::Stone1 => TileVec::new(4, 4),
			DetailType::WideBush0 => TileVec::new(16, 8),
		}
	}
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum TextureLayer {
	Background,
	Foreground,
}

pub trait IntoTextureLayer {
	fn to_layer(self) -> TextureLayer;
}

impl IntoTextureLayer for DetailType {
	fn to_layer(self) -> TextureLayer {
		match self {
			DetailType::Bush0 => TextureLayer::Background,
			DetailType::BushFlowers0 => TextureLayer::Background,
			DetailType::FloatingBush0 => TextureLayer::Foreground,
			DetailType::GrassStraws0 => TextureLayer::Foreground,
			DetailType::HangingBush0 => TextureLayer::Foreground,
			DetailType::Mountains0 => TextureLayer::Background,
			DetailType::Mountains1 => TextureLayer::Background,
			DetailType::Stone0 => TextureLayer::Background,
			DetailType::Stone1 => TextureLayer::Background,
			DetailType::WideBush0 => TextureLayer::Background,
		}
	}
}

pub fn texture_filenames_iter() -> impl Iterator<Item=String> {
	let textures = TextureId::iter()
		.map(TextureId::filepath);

	let animation_textures = AnimationId::iter()
		.flat_map(AnimationId::filepaths_iter);

	textures.chain(animation_textures)
}
