use crate::prelude::*;

const MIN_DAMAGE: u32 = 3;

const fn update_reference_mixin(s: FluidState) -> (i32, i32) {
	match s {
		FluidState::AtHand => (2, 200),
		FluidState::Free => (1, 200),
	}
}

impl Fluid {
	fn activity(&self) -> u32 {
		(self.reference_position - self.position).length() as u32
	}

	pub fn damage(&self) -> i32 {
		self.activity().max(MIN_DAMAGE) as i32
	}

	fn despawn_rate(&self) -> (u32, u32) {
		match self.state {
			FluidState::AtHand => (2, 10000),
			FluidState::Free => (10 * self.activity() / TILESIZE as u32, 10 * 200),
		}
	}

	pub fn check_despawn(&self) -> bool {
		// TODO make deterministic!
		let (rate, antirate) = self.despawn_rate();
		rand::random::<u32>() % (rate + antirate) <= rate
	}

	pub(in super) fn update_reference_position(&mut self) {
		let (new_mixin, old_mixin) = update_reference_mixin(self.state);
		self.reference_position = (self.position * new_mixin + self.reference_position * old_mixin) / (new_mixin + old_mixin);
	}

}