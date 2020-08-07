use crate::prelude::*;

macro_rules! setup {
	($($x:ident : $y:expr),*) => {

		#[derive(Copy, Clone, Debug)]
		#[repr(usize)]
		#[allow(unused)]
		pub enum TextureId {
			$($x),*
		}

		impl TextureState {
			pub fn new() -> TextureState {
				let mut textures = Vec::new();
				$(
					textures.push(Texture::from_file($y).expect("could not load file"));
				)*
				TextureState { textures }
			}
		}
	};
}

pub struct TextureState {
	textures: Vec<SfBox<Texture>>,
}

impl TextureState {
	#[allow(unused)]
	pub fn get_texture(&self, id: TextureId) -> &'_ Texture {
		&*self.textures[id as usize]
	}
}

setup!(
	BluePlayerIdle1: "res/images/player_blue/player_idle/player_idle1.png"
);
