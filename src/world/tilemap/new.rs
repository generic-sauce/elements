#[cfg(not(feature = "web-client"))]
mod native {
	use crate::prelude::*;

	impl TileMap {
		pub fn new(filename: &str) -> TileMap {
			use image::{GenericImageView, Rgba};
			let filename = res(filename);

			let image = image::open(filename).unwrap();
			let (width, height) = image.dimensions();
			let mut tiles = Vec::with_capacity((width * height) as usize);

			for y in (0..height).rev() {
				for x in 0..width {
					let tile = match image.get_pixel(x as u32, y as u32) {
						Rgba([255, 255, 255, 255]) => Tile::Void,
						Rgba([0, 0, 0, 255]) => Tile::Ground,
						c => panic!("tile color out of range! {:?}", c),
					};
					tiles.push(tile);
				}
			}

			let size = TileVec::new(width as i32, height as i32);
			TileMap {
				tiles,
				size,
			}
		}
	}
}

#[cfg(feature = "web-client")]
mod web {
	use crate::prelude::*;

	impl TileMap {
		pub fn new(filename: &str) -> TileMap {
			let width = 128;
			let height = 72;
			let mut tiles = vec![Tile::Void; (width * height) as usize];

			for y in 0..height {
				for x in 0..width {
					if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
						tiles[x + y * width] = Tile::Ground;
					}
				}
			}

			let size = TileVec::new(width as i32, height as i32);
			TileMap {
				tiles,
				size,
			}
		}
	}
}
