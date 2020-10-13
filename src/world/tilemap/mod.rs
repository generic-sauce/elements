mod update;

pub use update::*;

use crate::prelude::*;

pub const WALL_LIFETIME: u32 = 40;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tile {
	Void,
	Ground,
	Wall { owner: usize, remaining_lifetime: u32 },
}

#[derive(Serialize, Deserialize)]
pub struct TileMap {
	pub tiles: Vec<Tile>,
	pub size: TileVec,
}

impl TileMap {
	#[cfg(not(feature = "web-client"))]
	pub fn new() -> TileMap {
		use image::{GenericImageView, Rgba};
		let filename = res("map/map02.png");

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

	#[cfg(feature = "web-client")]
	pub fn new() -> TileMap {
		let width = 128;
		let height = 72;
		let mut tiles = vec![Tile::Void; (width * height) as usize];

		for y in 0..height {
			for x in 0..width {
				if x == 0 || y == 0 || x == width-1 || y == height-1 {
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

	pub fn reset(&mut self, handler: &mut impl EventHandler) {
		for x in &mut self.tiles {
			if let Tile::Wall { .. } = x {
				*x = Tile::Void;
				handler.tilemap_changed();
			}
		}
	}

	#[allow(unused)]
	pub fn get_mut(&mut self, v: TileVec) -> &'_ mut Tile {
		&mut self.tiles[(v.x + v.y * self.size.x) as usize]
	}

	#[allow(unused)]
	pub fn get(&self, v: TileVec) -> Tile {
		self.tiles.get((v.x + v.y * self.size.x) as usize)
			.cloned()
			.unwrap_or(Tile::Ground)
	}

	#[allow(unused)]
	pub fn set(&mut self, v: TileVec, tile: Tile) {
		self.tiles[(v.x + v.y * self.size.x) as usize] = tile;
	}

	pub fn check_solid(&self, v: impl Into<TileVec>) -> bool {
		self.get(v.into()).is_solid()
	}

	pub fn iter(&self) -> impl Iterator<Item=TileVec> + '_ {
		(0..self.size.x).flat_map(move |x| {
			(0..self.size.y).map(move |y| TileVec::new(x, y))
		})
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
