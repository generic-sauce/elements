use crate::prelude::*;

// TODO this thing is not used currently!

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
					let y = res($y);
					textures.push(Texture::from_file(&y).expect("could not load file"));
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
	Any: "images/checkerboard.png",
	BluePlayerIdle1: "images/player_blue/player_idle/player_idle1.png"
);
