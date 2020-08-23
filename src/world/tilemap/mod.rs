mod draw;

use crate::prelude::*;

#[derive(Clone, Copy)]
pub enum Tile {
	Void,
	Ground,
}

pub struct TileMap {
	pub tiles: Vec<Tile>,
	pub texture: SfBox<Texture>,
	pub size: TileVec,
}

impl TileMap {
	pub fn new(filename: &str) -> TileMap {
		let texture = Texture::from_file(filename).unwrap();
		let image = texture.copy_to_image().unwrap();
		let s = image.size();
		let mut tiles = Vec::with_capacity((s.x * s.y) as usize);

		for y in (0..s.y).rev() {
			for x in 0..s.x {
				let tile = match image.pixel_at(x as u32, y as u32) {
					Color { r: 255, g: 255, b: 255, a: 255 } => Tile::Void,
					Color { r: 0, g: 0, b: 0, a: 255 } => Tile::Ground,
					c => panic!("tile color out of range! {:?}", c),
				};
				tiles.push(tile);
			}
		}
		let texture = TileMap::create_texture(&tiles, s.into());

		TileMap {
			tiles,
			size: TileVec::new(s.x as i32, s.y as i32), // TODO make nicer
			texture: texture,
		}
	}

	#[allow(unused)]
	pub fn get_mut(&mut self, v: TileVec) -> &'_ mut Tile {
		&mut self.tiles[(v.x + v.y * self.size.x) as usize]
	}

	#[allow(unused)]
	pub fn get(&self, v: TileVec) -> Tile {
		self.tiles[(v.x + v.y * self.size.x) as usize]
	}

	fn create_texture(tiles: &Vec<Tile>, size: Vec2u) -> SfBox<Texture> {
		let mut pixels = Vec::new();
		for tile in tiles.iter() {

			let team = 0 as u8;
			let ground: u8 = match tile {
				Tile::Void => 0,
				Tile::Ground => 255,
			};
			let ratio = 0 as u8;

			pixels.push(ground);
			pixels.push(team);
			pixels.push(ratio);
			pixels.push(255 as u8);
		}

		let image = Image::create_from_pixels(size.x, size.y, &pixels).unwrap();
		Texture::from_image(&image).unwrap()
	}

	pub fn check_solid(&self, v: impl Into<TileVec>) -> bool {
		self.get(v.into()).is_solid()
	}
}

impl Tile {
	pub fn is_solid(self) -> bool {
		match self {
			Tile::Void => false,
			Tile::Ground => true,
		}
	}
}
