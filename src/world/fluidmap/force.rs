use crate::prelude::*;

// if fluids have a distance <= FLUID_AFFECT_DIST they may affect each other
pub const FLUID_AFFECT_DIST: i32 = 500; // in game coordinates

impl FluidMap {
	pub(in super) fn apply_forces(&mut self) {
		// gravity
		for grid_field in self.grid.iter_mut() {
			for f in grid_field.into_iter() {
				f.velocity.y -= GRAVITY;
			}
		}

		// TODO push & pull fluid reactions
	}
}
