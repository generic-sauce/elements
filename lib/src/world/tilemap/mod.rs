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
	pub fn new(filename: &str) -> TileMap {
		let image = Image::from_file(filename).unwrap();
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

		TileMap {
			tiles,
			size: s,
		}
	}

	#[must_use]
	pub fn reset(&mut self) -> Vec<Command> {
		for x in &mut self.tiles {
			if let Tile::Wall { .. } = x {
				*x = Tile::Void;
			}
		}

		vec![Command::UpdateTileMapTexture]
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
		(0..self.size.x).map(move |x| {
			(0..self.size.y).map(move |y| TileVec::new(x, y))
		}).flatten()
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
