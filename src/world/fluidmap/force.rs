use crate::prelude::*;

// if fluids have a distance <= FLUID_AFFECT_DIST they may affect each other
pub const FLUID_AFFECT_DIST: i32 = 500; // in game coordinates

impl FluidMap {
	pub(in super) fn apply_forces(&self) -> impl Iterator<Item=Fluid> + '_ {
		self.iter().map(|f| {
			Fluid {
				velocity: f.velocity - GameVec::new(0, GRAVITY),
				..f.clone()
			}
		})
		// TODO push & pull fluid reactions
	}
}
