use crate::prelude::*;

const FLUID_GRAVITY: i32 = GRAVITY / 3;
const fn free_drag(x: i32) -> i32 { x * 255 / 256 }
const fn hand_drag(x: i32) -> i32 { x * 24 / 32 }
pub const MAX_FLUID_SPEED: i32 = 500;

impl FluidMap {
	pub(in super) fn apply_forces(&mut self, t: &TileMap, players: &[Player; 2], frame_id: u32) {
		for f in self.iter_mut_notranslate() {
			// gravity
			f.velocity -= GameVec::new(0, FLUID_GRAVITY);

			if let FluidState::AtHand = f.state {
				let cursor = players[f.owner].cursor_position();
				apply_cursor_steering(f, cursor);
			}

			// drag
			f.velocity = f.velocity.map(
				match f.state {
					FluidState::Free => free_drag,
					FluidState::AtHand => hand_drag,
				}
			);

			// noise
			f.velocity += (
				noise(frame_id, f.id, 0),
				noise(frame_id, f.id, 1),
			);

			f.velocity = f.velocity.length_clamped(MAX_FLUID_SPEED);
		}
	}
}

fn apply_cursor_steering(f: &mut Fluid, cursor: GameVec) {
	const MAX_SPEED: i32 = 600;
	const MAX_FORCE: i32 = 260;

	let desired_velocity = (cursor - f.position).length_clamped(MAX_SPEED);
	let steering = (desired_velocity - f.velocity)
		.length_clamped(MAX_FORCE);
	f.velocity = (f.velocity + steering).length_clamped(MAX_SPEED);
}

// returns -1 or 1
fn noise(fluid_id: u32, frame_id: u32, num: u32) -> i32 {
	use rand::{SeedableRng, RngCore};
	use rand_xorshift::XorShiftRng;

	let seed = (fluid_id + num * 7) as u64;
	let seed = XorShiftRng::seed_from_u64(seed).next_u64() / 2 + frame_id as u64; // the / 2 prevents overflows
	let result = XorShiftRng::seed_from_u64(seed).next_u32();
	match result % 2 {
		0 => -1,
		_ => 1,
	}
}
