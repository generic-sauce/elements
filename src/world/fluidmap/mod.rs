mod draw;
use crate::prelude::*;

mod force;
mod physics;

pub const NUM_FLUID_CELLS: Vec2i = Vec2i::new(200, 200);

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
		let fluids: Vec<Fluid> = self.grid.iter_mut().map(|cell|
				cell.drain(..)
			).flatten()
			.collect();

		for f in fluids {
			let cell_pos = f.position * NUM_FLUID_CELLS / t.size.to_i() / TILESIZE;
			let i = (cell_pos.x + cell_pos.y * NUM_FLUID_CELLS.x) as usize;
			self.grid[i].push(f);
		}
	}
}
