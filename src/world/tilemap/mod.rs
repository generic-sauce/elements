mod draw;

use crate::prelude::*;

pub const WALL_LIFETIME: u32 = 20;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
	Void,
	Ground,
	Wall { owner: usize, remaining_lifetime: u32 }, // TODO this feels terrible.
}

pub struct TileMap {
	pub tiles: Vec<Tile>,
	pub size: TileVec,
	pub texture: SfBox<Texture>,
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
		let s = TileVec::new(s.x as i32, s.y as i32); // TODO make nicer
		let texture = TileMap::create_texture(&tiles, s);

		TileMap {
			tiles,
			size: s,
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

	#[allow(unused)]
	pub fn set(&mut self, v: TileVec, tile: Tile) {
		self.tiles[(v.x + v.y * self.size.x) as usize] = tile;
		self.update_texture();
	}

	pub fn update_texture(&mut self) {
		self.texture = TileMap::create_texture(&self.tiles, self.size);
	}

	fn create_texture(tiles: &Vec<Tile>, size: TileVec) -> SfBox<Texture> {
		let mut pixels = Vec::new();
		for &tile in tiles.iter() {

			let team: u8 = match tile {
				Tile::Wall { owner, .. } => owner as u8 * 255, // TODO maybe owner should be u8 generally
				 _ => 0,
			};
			let ground: u8 = match tile {
				Tile::Void => 0,
				_ => 255,
			};
			let ratio: u8 = match tile {
				Tile::Wall { .. } => 255, // TODO correct?
				_ => 0,
			};

			pixels.push(ground);
			pixels.push(team);
			pixels.push(ratio);
			pixels.push(255 as u8);
		}

		let image = Image::create_from_pixels(size.x as u32, size.y as u32, &pixels).unwrap();
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
			_ => true,
		}
	}
}
