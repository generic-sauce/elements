mod update;

pub use update::*;

use crate::prelude::*;

pub const WALL_LIFETIME: u32 = 40;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Brick {
	pub owner: usize,
	pub remaining_lifetime: u32,
	pub velocity: GameVec,
	pub position: GameVec
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tile {
	Void,
	Ground,
	Wall { owner: usize, remaining_lifetime: u32 },
	Brick(Brick),
}

#[derive(Serialize, Deserialize)]
pub struct TileMap {
	pub tiles: Vec<Tile>,
	pub size: TileVec,
}

impl World {
	pub fn tick_tilemap(&mut self, handler: &mut impl EventHandler) {
		TileMap::iter(self.tilemap.size)
			.for_each(|i| self.tilemap.apply_brick_move(i, &self.players));
		handler.tilemap_changed();
	}
}

impl TileMap {
	pub fn new(filename: &str) -> TileMap {
		use image::{GenericImageView, Rgba};

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

	pub fn reset(&mut self, handler: &mut impl EventHandler) {
		for x in &mut self.tiles {
			if let Tile::Wall { .. } = x {
				*x = Tile::Void;
				handler.tilemap_changed();
			}
		}
	}

	pub fn bick_next_position(index: TileVec, brick: Brick) -> GameVec {
		brick.position + brick.velocity
	}

	pub fn apply_brick_move(&mut self, i: TileVec, players: &[Player]) {
		let tile = self.get(i);
		let tile = match tile {
			Tile::Brick(brick) => {
				let mut brick = brick;
				let position = TileMap::bick_next_position(i, brick);
				let tile_position: TileVec = position.into();
				brick.position += brick.velocity;
				println!("brick");

				if i != tile_position {
					// move the brick
					println!("move");

					self.set(tile_position, Tile::Brick(brick));
					Tile::Void
				} else {
					// dot not move the brick

					println!("not");
					Tile::Brick(brick)
				}
			},
			_ => tile
		};
		self.set(i, tile);
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

	pub fn iter(size: TileVec) -> impl Iterator<Item=TileVec> {
		(0..size.x).map(move |x| {
			(0..size.y).map(move |y| TileVec::new(x, y))
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
