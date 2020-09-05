use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct FluidMapUpdate {
	pub fluids: Vec<Fluid>,
	pub next_id: u32,
	pub spawn_counter: u32,
}

impl FluidMap {
	pub fn update(&self) -> FluidMapUpdate {
		FluidMapUpdate {
			fluids: self.iter().cloned().collect(),
			next_id: self.next_id,
			spawn_counter: self.spawn_counter
		}
	}

	pub fn apply_update(&mut self, u: FluidMapUpdate) {
		// This exists to generate a compiler error whenever a field will be added to FluidMap.
		*self = FluidMap {
			grid: FluidMap::mk_grid(u.fluids.into_iter(), self.size),
			size: self.size,
			next_id: u.next_id,
			spawn_counter: u.spawn_counter,
		};
	}
}
