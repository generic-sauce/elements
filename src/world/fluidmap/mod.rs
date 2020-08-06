mod draw;
use crate::prelude::*;

mod force;
mod physics;

pub const NUM_FLUID_CELLS: Vec2i = Vec2i::new(20, 20);

pub enum FluidState {
	Owned,
	AtHand,
	Free,
}

pub struct Fluid {
	pub state: FluidState,
	pub owner: usize,
	pub velocity: Vec2i,
	pub position: Vec2i,
}

pub struct FluidMap {
	grid: Vec<Vec<Fluid>>,
}

impl FluidMap {
	pub fn new() -> FluidMap {
		let mut m = FluidMap { grid: (0..(NUM_FLUID_CELLS.x * NUM_FLUID_CELLS.y)).map(|_| Vec::new()).collect() };

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
		let mut grid: Vec<Vec<Fluid>> = (0..(NUM_FLUID_CELLS.x * NUM_FLUID_CELLS.y)).map(|_| Vec::new()).collect();

		let fluids = self.grid.iter_mut().map(|cell|
				cell.drain(..)
			).flatten();

		for f in fluids {
			let cell_pos = f.position * NUM_FLUID_CELLS / t.size.to_i() / TILESIZE;
			let i = (cell_pos.x + cell_pos.y * NUM_FLUID_CELLS.x) as usize;
			grid[i].push(f);
		}

		self.grid = grid;
	}
}
