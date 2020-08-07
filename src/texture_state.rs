use crate::prelude::*;

macro_rules! setup {
	($($x:ident : $y:expr),*) => {

		#[derive(Copy, Clone, Debug)]
		#[repr(usize)]
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
	pub fn get_texture(&self, id: TextureId) -> &'_ Texture {
		&*self.textures[id as usize]
	}
}

setup!(
	BluePlayerIdle1: "res/images/player_blue/player_idle/player_idle1.png",
	BluePlayerIdle2: "res/images/player_blue/player_idle/player_idle2.png",
	BluePlayerIdle3: "res/images/player_blue/player_idle/player_idle3.png",
	BluePlayerIdle4: "res/images/player_blue/player_idle/player_idle4.png",
	BluePlayerIdle5: "res/images/player_blue/player_idle/player_idle5.png"
);
