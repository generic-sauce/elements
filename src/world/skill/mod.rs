use crate::prelude::*;

impl World {
	pub(in super) fn handle_skills(&mut self, inputs: &mut [Box<dyn Input>; 2]) {
		for p in 0..2 {
			if inputs[p].attack1() {
				self.fluidmap.iter_mut_notranslate()
					.for_each(|f| f.state = FluidState::Free)
			}

			if inputs[p].attack2() {
				self.fluidmap.iter_mut_notranslate()
					.take(3)
					.for_each(|f| f.state = FluidState::Free)
			}
		}
	}
}
