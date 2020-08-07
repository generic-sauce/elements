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

			// drag
			let velocity = velocity - velocity / 10;

			// neighbour-affection
			let velocity = velocity + neighbours
				.map(|n| affect(f, n))
				.sum::<GameVec>();

			let velocity = velocity.clamped(-200, 200);

			Fluid {
				velocity,
				..f.clone()
			}
		})
	}
}

fn affect(f: &Fluid, n: &Fluid) -> GameVec {
	let v = (n.position - f.position);

	if v.magnitude_sqr() <= 200 * 200 {
		v / 10
	} else {
		(v / 20) * (-1)
	}
}
