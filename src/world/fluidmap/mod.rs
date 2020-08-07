use crate::prelude::*;

mod draw;
mod physics;

pub mod force;
pub use self::force::*;

pub enum FluidState {
	Owned,
	AtHand,
	Free,
}

pub struct Fluid {
	pub state: FluidState,
	pub owner: usize,
	pub velocity: GameVec,
	pub position: GameVec,
}

pub struct FluidMap {
	grid: Vec<Vec<Fluid>>,
}

impl FluidMap {
	pub fn new(tilesize: Vec2u) -> FluidMap { // TODO accept a TileVec here!
		let fluidmap_size = fluidmap_size(tilesize);
		let mut m = FluidMap { grid: (0..(fluidmap_size.x * fluidmap_size.y)).map(|_| Vec::new()).collect() };

		// TODO remove
		m.grid[0].push(Fluid {
			state: FluidState::Free,
			owner: 0,
			velocity: 0.into(),
			position: 13000.into(),
		});

		m
	}

	pub fn tick(&mut self, t: &TileMap) {
		self.apply_forces();
		self.move_by_velocity(t);
		self.update_grid(t);
	}

	fn update_grid(&mut self, t: &TileMap) {
		let fluidmap_size = fluidmap_size(t.size);
		let mut grid: Vec<Vec<Fluid>> = (0..(fluidmap_size.x * fluidmap_size.y)).map(|_| Vec::new()).collect();

		let fluids = self.grid.iter_mut().map(|cell|
				cell.drain(..)
			).flatten();

		for f in fluids {
			let fluid_pos = f.position.to_fluid();
			let i = (fluid_pos.x + fluid_pos.y * fluidmap_size.x) as usize;
			grid[i].push(f);
		}

		self.grid = grid;
	}

	pub fn iter(&self) -> impl Iterator<Item=&Fluid> + '_ {
		self.grid.iter()
			.map(|x| x.iter())
			.flatten()
	}
}

fn fluidmap_size(tilemap_size: Vec2u) -> FluidVec {
	let tilemap_size = TileVec::new(tilemap_size.x as i32, tilemap_size.y as i32); // number of tiles
	let gamesize = tilemap_size.to_game(); // number of game-tiles
	let fluidmap_size = gamesize.to_fluid(); // number of fluid-tiles
	fluidmap_size
}
