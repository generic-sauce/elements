use crate::prelude::*;

macro_rules! setup {
	($($x:ident : $y:expr),*) => {

		#[derive(Copy, Clone, Debug)]
		#[repr(usize)]
		pub enum FontId {
			$($x),*
		}

		impl FontState {
			pub fn new() -> FontState {
				let mut fonts = Vec::new();
				$(
					fonts.push(Font::from_file($y).expect("could not load file"));
				)*
				FontState { fonts }
			}
		}
	};
}

pub struct FontState {
	fonts: Vec<SfBox<Font>>,
}

impl FontState {
	pub fn get_font(&self, id: FontId) -> &'_ Font {
		&*self.fonts[id as usize]
	}
}

setup!(
	DefaultFont: "res/dashing_unicorn.ttf"
);
