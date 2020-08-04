mod render;

use crate::prelude::*;

#[derive(Clone, Copy)]
pub enum Tile {
	Void,
	Ground,
}

pub struct TileMap {
	tiles: Vec<Tile>,
    size: Vec2u,
}

impl TileMap {
	pub fn new(filename: &str) -> TileMap {
		let texture = Texture::from_file(filename).unwrap();
		let image = texture.copy_to_image().unwrap();
        let s = image.size();
		let mut tiles = Vec::with_capacity((s.x * s.y) as usize);

		for y in 0..s.y {
			for x in 0..s.x {
				let tile = match image.pixel_at(x as u32, y as u32) {
					Color { r: 255, g: 255, b: 255, a: 255 } => Tile::Void,
					Color { r: 0, g: 0, b: 0, a: 255 } => Tile::Ground,
					c => panic!("tile color out of range! {:?}", c),
				};
				tiles.push(tile);
			}
		}

		TileMap {
			tiles,
            size: s.into(),
		}
	}

	pub fn get_mut(&mut self, v: Vec2u) -> &'_ mut Tile {
		&mut self.tiles[(v.x + v.y * self.size.x) as usize]
	}

	pub fn get(&self, v: Vec2u) -> Tile {
		self.tiles[(v.x + v.y * self.size.x) as usize]
	}
}
