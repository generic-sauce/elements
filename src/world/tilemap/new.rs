use crate::prelude::*;

type RGBA = [u8; 4];

#[derive(Serialize, Deserialize)]
struct TileMapImage {
	pixels: Vec<Vec<RGBA>>, // pixels[x][y]; pixels[0][0] is left-bot
	size: TileVec,
}

impl TileMap {
	pub fn new(filename: &str) -> TileMap {
		let TileMapImage { pixels, size } = TileMapImage::new(filename);
		let mut tiles = Vec::with_capacity((size.x * size.y) as usize);

		for y in 0..size.y {
			for x in 0..size.x {
				let tile = match pixels[x as usize][y as usize] {
					[255, 255, 255, 255] => Tile::Void,
					[0, 0, 0, 255] => Tile::Ground,
					c => panic!("tile color out of range! {:?}", c),
				};
				tiles.push(tile);
			}
		}

		TileMap {
			tiles,
			size,
		}
	}
}

#[cfg(not(feature = "web-client"))]
mod native {
	use crate::prelude::*;
	use super::{RGBA, TileMapImage};

	impl TileMapImage {
		pub fn new(filename: &str) -> Self {
			use image::{GenericImageView, Rgba};

			let filename = res(filename);

			let image = image::open(filename).unwrap();
			let (width, height) = image.dimensions();
			let mut pixels: Vec<Vec<RGBA>> = (0..width)
				.map(|_|
					(0..height).map(|_| [0; 4])
						       .collect()
				).collect();

			for y in 0..height {
				for x in 0..width {
					let Rgba(rgba) = image.get_pixel(x as u32, y as u32);
					pixels[x as usize][(height - y - 1) as usize] = rgba;
				}
			}

			let size = TileVec::new(width as i32, height as i32);
			Self {
				pixels,
				size,
			}
		}
	}
}

#[cfg(feature = "web-client")]
mod web {
	use crate::prelude::*;
	use super::TileMapImage;

	#[wasm_bindgen]
	extern {
		fn new_tilemap_image(filename: &str) -> JsValue;
	}

	impl TileMapImage {
		pub fn new(filename: &str) -> Self {
			new_tilemap_image(filename).into_serde().unwrap()
		}
	}
}
