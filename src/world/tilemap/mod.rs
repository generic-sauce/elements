mod update;
pub use update::*;

mod src;
pub use src::*;

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
	pub fn new(src: impl MapSrc) -> TileMap {
		let TileMapImage { pixels, size } = src.image();
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
		(0..self.size.y).flat_map(move |y| {
			(0..self.size.x).map(move |x| TileVec::new(x, y))
		})
	}
}

impl Tile {
	pub fn is_solid(self) -> bool {
		!matches!(self, Tile::Void)
	}
}
