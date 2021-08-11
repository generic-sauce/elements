mod update;
pub use update::*;

mod tilemap_image;
pub use tilemap_image::*;

use crate::prelude::*;

pub const WALL_LIFETIME: u32 = 40;
pub const WALL_IGNORE_FRIENDLY_FLUIDS_TIME: u32 = 10;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tile {
	Void,
	Ground,
	Wall { team: u8, remaining_lifetime: u32 },
}

#[derive(Serialize, Deserialize)]
pub struct TileMap {
	pub tiles: Vec<Tile>,
	pub size: TileVec,
	pub spawn_points: [Vec<TileVec>; 2],
	pub details: Vec<(GameVec, DetailType)>
}

#[derive(Serialize, Deserialize)]
pub enum DetailType {
	Bush0,
	BushFlowers0,
	FloatingBush0,
	GrassStraws0,
	HangingBush0,
	Mountains0,
	Mountains1,
	Stone0,
	Stone1,
	WideBush0,
}

impl TileMap {
	pub fn new(src: &TileMapImage) -> TileMap {
		let TileMapImage { pixels, size } = src;
		let mut tiles = Vec::with_capacity((size.x * size.y) as usize);
		let mut spawn_points = [Vec::new(), Vec::new()];

		let mut details = Vec::new();

		for y in 0..size.y {
			for x in 0..size.x {
				let tile = match pixels[x as usize][y as usize] {
					[255, 255, 255, 255] => Tile::Void,
					[0, 0, 0, 255] => Tile::Ground,
					[0, 0, 255, 255] => {
						spawn_points[0].push(TileVec::new(x, y));
						Tile::Void
					}
					[255, 0, 0, 255] => {
						spawn_points[1].push(TileVec::new(x, y));
						Tile::Void
					}
					[0, 128, 0, 255] => {
						details.push((GameVec::from(TileVec::new(x, y)), DetailType::Bush0));
						Tile::Void
					}
					[0, 179, 0, 255] => {
						details.push((GameVec::from(TileVec::new(x, y)), DetailType::BushFlowers0));
						Tile::Void
					}
					[0, 230, 0, 255] => {
						details.push((GameVec::from(TileVec::new(x, y)), DetailType::FloatingBush0));
						Tile::Void
					}
					[26, 255, 26, 255] => {
						details.push((GameVec::from(TileVec::new(x, y)), DetailType::GrassStraws0));
						Tile::Void
					}
					[77, 255, 77, 255] => {
						details.push((GameVec::from(TileVec::new(x, y)), DetailType::HangingBush0));
						Tile::Ground
					}
					[128, 64, 0, 255] => {
						details.push((GameVec::from(TileVec::new(x, y)), DetailType::Mountains0));
						Tile::Void
					}
					[179, 89, 0, 255] => {
						details.push((GameVec::from(TileVec::new(x, y)), DetailType::Mountains1));
						Tile::Void
					}
					[230, 115, 0, 255] => {
						details.push((GameVec::from(TileVec::new(x, y)), DetailType::Stone0));
						Tile::Void
					}
					[255, 140, 26, 255] => {
						details.push((GameVec::from(TileVec::new(x, y)), DetailType::Stone1));
						Tile::Void
					}
					[255, 166, 77, 255] => {
						details.push((GameVec::from(TileVec::new(x, y)), DetailType::WideBush0));
						Tile::Void
					}
					c => panic!("tile color out of range! {:?}", c),
				};
				tiles.push(tile);
			}
		}

		assert!(!spawn_points[0].is_empty(), "no team 0 spawnpoint found in map");
		assert!(!spawn_points[1].is_empty(), "no team 1 spawnpoint found in map");

		TileMap {
			tiles,
			size: *size,
			spawn_points,
			details,
		}
	}

	pub fn get_spawn_positions(&self, team: u8) -> &Vec<TileVec> {
		&self.spawn_points[team as usize]
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
