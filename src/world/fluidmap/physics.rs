use crate::prelude::*;

const BUFFER_DIST: i32 = FLUID_MIN_DIST + 10;

enum Collision {
	Fluid { idx: usize, change: GameVec },
	TileX { change: GameVec },
	TileY { change: GameVec },
}

impl Collision {
	fn change_len_sqr(&self) -> i32 {
		match self {
			Collision::Fluid { change, .. }
			| Collision::TileX { change  }
			| Collision::TileY { change  } => change.length_squared(),
		}
	}
}

fn tile_reflect(x: i32) -> i32 { -x / 3 } // TODO add some randomness.

impl FluidMap {
	pub(in super) fn tick_physics(&mut self, t: &TileMap) {
		let n = self.grid.len();

		let mut ignore_mask = bitvec::vec::BitVec::<bitvec::order::Lsb0>::new();
		ignore_mask.resize(n, false);

		for i in 0..n {
			if ignore_mask[i] { continue; }

			let mut f = if let Some(f) = self.grid[i].take() { f } else { continue };
			self.move_fluid(&mut f, t);

			let new_i = self.add_fluid(f);
			ignore_mask.set(new_i, true);
		}
	}

	fn find_next_collision(&self, pos: GameVec, remaining_vel: GameVec) -> Option<Collision> {
		let fluid_iter = self.neighbours_of_pos(pos, BUFFER_DIST + remaining_vel.length()) // TODO find a smarter neighbour-set
			.filter_map(|f| {
				// from f to self
				// from f to self
				let pos_diff = pos - f.position;

				let pos_num = (pos_diff.length() - BUFFER_DIST).max(0);
				let vel_num = remaining_vel.length();

				if vel_num > 0 && pos_diff.dot(remaining_vel) <= 0 {
					let idx = FluidMap::index(self.size, f.position.into());
					let change = remaining_vel * pos_num / vel_num;
					Some(Collision::Fluid { idx, change })
				} else {
					None
				}
			});

		let tile_x_iter = (|| {
			if remaining_vel.x != 0 {
				let route = GameVec::new(route(remaining_vel.x, pos.x), route(remaining_vel.y, pos.y));

				let ychange = route.x.abs() * remaining_vel.y / remaining_vel.x.abs();
				if ychange.abs() > 1000 { return None; } // in this case the other coord is preferred, and it causes an overflow // TODO un-hardcode

				let change = GameVec::new(route.x, ychange);

				Some(Collision::TileX { change })
			} else { None }
		})().into_iter();

		let tile_y_iter = (|| {
			if remaining_vel.y != 0 {
				let route = GameVec::new(route(remaining_vel.x, pos.x), route(remaining_vel.y, pos.y));

				let xchange = route.y.abs() * remaining_vel.x / remaining_vel.y.abs();
				if xchange.abs() > 1000 { return None; } // in this case the other coord is preferred, and it causes an overflow // TODO un-hardcode

				let change = GameVec::new(xchange, route.y);

				Some(Collision::TileY { change })
			} else { None }
		})().into_iter();

		fluid_iter.chain(tile_x_iter).chain(tile_y_iter)
			.min_by_key(|c| c.change_len_sqr())
			.filter(|c| c.change_len_sqr() <= remaining_vel.length_squared())
	}

	fn handle_collision(&mut self, f: &mut Fluid, c: Collision, remaining_vel: &mut GameVec, t: &TileMap) {
		match c {
			Collision::TileX { change } => {
				let change_ex = change + (remaining_vel.x.signum(), 0);
				if t.check_solid(f.position + change_ex) {
					assert!(!t.check_solid(f.position + change));

					*remaining_vel -= change;
					f.position += change;

					remaining_vel.x = 0;
					f.velocity.x = tile_reflect(f.velocity.x);
				} else {
					*remaining_vel -= change_ex;
					f.position += change_ex;
				}
			},
			Collision::TileY { change } => {
				let change_ex = change + (0, remaining_vel.y.signum());
				if t.check_solid(f.position + change_ex) {
					assert!(!t.check_solid(f.position + change));

					*remaining_vel -= change;
					f.position += change;

					remaining_vel.y = 0;
					f.velocity.y = tile_reflect(f.velocity.y);
				} else {
					*remaining_vel -= change_ex;
					f.position += change_ex;
				}
			},
			Collision::Fluid { idx, change } => {
				f.position += change;

				let other = self.grid[idx].as_mut().unwrap();

				let other_to_us = f.position - other.position;
				let vel_other_to_us = other.velocity - f.velocity;

				let sqrt = |x: i32| (x as f64).sqrt() as i32;
				let overlap = sqrt(vel_other_to_us.dot(other_to_us).max(0));
				let reflect_overlap = overlap / 2 + overlap / 5; // a little more than the normal overlap/2 in order to bounce back

				f.velocity += other_to_us.with_length(reflect_overlap);
				other.velocity -= other_to_us.with_length(reflect_overlap);

				let rem_len = (remaining_vel.length() - change.length()).max(0);
				*remaining_vel = f.velocity.with_length(rem_len);

				// TODO this should never happen!
				if change == 0.into() {
					*remaining_vel = 0.into();
				}

			}
		}
	}

	fn move_fluid(&mut self, f: &mut Fluid, t: &TileMap) {
		if t.check_solid(f.position) {
			#[cfg(debug_assertions)]
			println!("A fluid is glitched.");
			return;
		}

		let mut remaining_vel = f.velocity;

		while let Some(coll) = self.find_next_collision(f.position, remaining_vel) {
			self.handle_collision(f, coll, &mut remaining_vel, t);
		}
		f.position += remaining_vel;
	}
}

// returns the change required to touch, but not collide the next tile
fn route(velocity: i32, pos: i32) -> i32 {
	if velocity < 0 {
		-(pos % TILESIZE)
	} else {
		(TILESIZE-1) - (pos % TILESIZE)
	}
}

