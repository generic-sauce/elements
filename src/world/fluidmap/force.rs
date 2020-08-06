use crate::prelude::*;

impl FluidMap {
	pub(in super) fn apply_forces(&mut self) {
		for grid_field in self.grid.iter_mut() {
			for f in grid_field.into_iter() {
				f.velocity.y -= GRAVITY;
			}
		}

		// TODO push & pull fluid reactions
	}
}
