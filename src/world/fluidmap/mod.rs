use crate::prelude::*;

mod force;
mod physics;

const NUM_FLUID_CELLS: Vec2i = Vec2i::new(20, 20);

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
			position: 4500.into(),
		});

		m
	}

	pub fn draw(&self, context: &mut Context) {
		context.draw_fluids(self);
	}

	pub fn tick(&mut self, t: &TileMap) {
		self.apply_forces();
		self.move_by_velocity(t);
		self.update_grid();
	}

	fn update_grid(&mut self) {
		// TODO
	}
}
