use crate::prelude::*;

mod draw;
mod physics;

pub mod force;
pub use self::force::*;

#[derive(Clone)]
pub enum FluidState {
	Owned,
	AtHand,
	Free,
}

#[derive(Clone)]
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
	pub fn new(tilemap_size: TileVec) -> FluidMap {
		let iter =
			Some(Fluid {
				state: FluidState::Free,
				owner: 0,
				velocity: 0.into(),
				position: 13000.into(),
			}).into_iter();

		FluidMap {
			grid: FluidMap::mk_grid(iter, tilemap_size),
		}
	}

	pub fn tick(&mut self, t: &TileMap) {
		self.apply_forces();
		self.move_by_velocity(t);
	}

	fn mk_grid(fluids: impl Iterator<Item=Fluid>, tilemap_size: TileVec) -> Vec<Vec<Fluid>> {
		let fluidmap_size = fluidmap_size(tilemap_size);
		let mut grid: Vec<Vec<Fluid>> = (0..(fluidmap_size.x * fluidmap_size.y)).map(|_| Vec::new()).collect();

		for f in fluids {
			let fluid_pos = f.position.to_fluid();
			let i = (fluid_pos.x + fluid_pos.y * fluidmap_size.x) as usize;
			grid[i].push(f);
		}

		grid
	}

	pub fn iter(&self) -> impl Iterator<Item=&Fluid> + '_ {
		self.grid.iter()
			.map(|x| x.iter())
			.flatten()
	}

	pub fn neighbours(&self, f: &Fluid) -> impl Iterator<Item=&Fluid> + '_ {
		// TODO only find neighbours which have distance <= FLUID_AFFECT_DIST
		self.iter()
	}
}

fn fluidmap_size(tilemap_size: TileVec) -> FluidVec {
	let tilemap_size = TileVec::new(tilemap_size.x as i32, tilemap_size.y as i32); // number of tiles
	let gamesize = tilemap_size.to_game(); // number of game-tiles
	let fluidmap_size = gamesize.to_fluid(); // number of fluid-tiles
	fluidmap_size
}
