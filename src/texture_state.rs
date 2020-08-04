use sfml::system::SfBox;
use sfml::graphics::Texture;

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
	PlayerIdle1: "res/player_idle1.png"
);
