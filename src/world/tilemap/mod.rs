mod update;
pub use update::*;

mod new;
pub use new::*;

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
