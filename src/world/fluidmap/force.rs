use crate::prelude::*;

// if fluids have a distance <= FLUID_AFFECT_DIST they may affect each other
pub const FLUID_AFFECT_DIST: i32 = 500; // in game coordinates
const DESIRED_FLUID_DIST: i32 = 400;

const FLUID_GRAVITY: i32 = GRAVITY / 3;
const fn free_drag(x: i32) -> i32 { x * 255 / 256 }
const fn hand_drag(x: i32) -> i32 { x * 15 / 16 }
const CURSOR_PULL: i32 = 220;

struct Force { // this represents a constraint saying that velocity projected onto goal should be larger than goal
	priority: i32,
	goal: GameVec,
}

impl FluidMap {
	pub(in super) fn apply_forces<'a>(&'a self, t: &'a TileMap, players: &'a [Player; 2]) -> impl Iterator<Item=Fluid> + 'a {
		self.iter().map(move |f| {
			// gravity
			let velocity = f.velocity - GameVec::new(0, FLUID_GRAVITY);

			// drag
			let velocity = velocity.map(
				match f.state {
					FluidState::Free => free_drag,
					FluidState::AtHand => hand_drag,
				}
			);

			// forces
			let iter = self.neighbours(f).map(|n| neighbour_force(f, n))
				.chain(
					if let FluidState::AtHand = f.state {
						let cursor = players[f.owner].cursor_position();
						Some(cursor_force(f, cursor))
					} else { None }
						.into_iter()
			);

			let velocity = calc_vel(velocity, iter);

			Fluid {
				velocity,
				..f.clone()
			}
		})
	}
}

fn calc_vel(old_velocity: GameVec, forces: impl Iterator<Item=Force>) -> GameVec {
	const ACCURACY: i32 = 5;

	let forces: Vec<_> = forces.collect();

	let mut prios: Vec<_> = forces.iter()
		.map(|x| x.priority)
		.collect();
	prios.sort_unstable();
	prios.dedup();

	let mut result = old_velocity;

	for p in prios {
        let filtered: Vec<&Force> = forces.iter().filter(|x| x.priority == p).collect();
		for _ in 0..ACCURACY {
			for &force in &filtered {
				if force.goal.dot(force.goal) > force.goal.dot(result) {
					result += force.goal / ACCURACY;
				}
			}
		}
	}

    result
}

fn sqrt(x: i32) -> i32 {
	(x as f32).sqrt() as i32
}

fn neighbour_force(f: &Fluid, n: &Fluid) -> Force {
	let to_neighbour = n.position - f.position;
	if to_neighbour.as_short_as(DESIRED_FLUID_DIST) {
		// push

		let value = sqrt(sqrt(DESIRED_FLUID_DIST - to_neighbour.length())) * 30;
		Force {
			priority: 10,
			goal: to_neighbour.with_length(value) * (-1),
		}
	} else {
		// pull

		let value = 4;
		Force {
			priority: 3,
			goal: to_neighbour.with_length(value),
		}
	}
}

fn cursor_force(f: &Fluid, cursor: GameVec) -> Force {
    Force {
		priority: 5,
		goal: (cursor - f.position).with_length(CURSOR_PULL), // TODO this should not be a constant force I assume
	}
}