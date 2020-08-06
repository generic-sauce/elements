use crate::prelude::*;

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
		FluidMap {
			grid: (0..(NUM_FLUID_CELLS.x * NUM_FLUID_CELLS.y)).map(|_| Vec::new()).collect()
		}
	}
}
