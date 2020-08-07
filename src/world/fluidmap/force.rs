use crate::prelude::*;

// if fluids have a distance <= FLUID_AFFECT_DIST they may affect each other
pub const FLUID_AFFECT_DIST: i32 = 500; // in game coordinates

impl FluidMap {
	pub(in super) fn apply_forces(&self) -> impl Iterator<Item=Fluid> + '_ {
		self.iter().map(move |f| {
			let neighbours = self.neighbours(f);

			let velocity = f.velocity;

			// gravity
			let velocity = velocity - GameVec::new(0, GRAVITY);

			// neighbour-affection
			let velocity = velocity + neighbours
				.map(|n| (f.position - n.position) / 100)
				.sum::<GameVec>();

			Fluid {
				velocity,
				..f.clone()
			}
		})
		// TODO push & pull fluid reactions
	}
}
