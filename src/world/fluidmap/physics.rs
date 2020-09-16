use crate::prelude::*;

enum XY {
	X,
	Y,
}

enum Collision {
	Fluid { idx: usize },
	Tile(XY),
}

fn reflect(x: i32) -> i32 { -x / 3 } // TODO add some randomness.

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

	fn find_next_collision(&self, pos: GameVec, velocity: GameVec, remaining_vel: GameVec) -> Option<Collision> {
		/*
		let fluid_iter = None.into_iter();

		let tile_x_iter =
			Some(Collision::Tile(XY::X))
				.filter(|_| remaining_vel.x != 0)
				.into_iter();

		let tile_y_iter =
			Some(Collision::Tile(XY::Y))
				.filter(|_| remaining_vel.y != 0)
				.into_iter();

		fluid_iter.chain(tile_x_iter).chain(tile_y_iter)
			.min_by_key(|c| c.change_until_touch.length_squared())
			.filter(|c| c.change_until_touch.length_squared() <= remaining_vel.length_squared())
		 */
		unimplemented!()
	}

	fn handle_collision(&mut self, f: &mut Fluid, c: Collision, remaining_vel: &mut GameVec, t: &TileMap) {
		match c {
			Collision::Tile(XY::X) => {
				assert!(remaining_vel.x != 0);

				let xroute = route(remaining_vel.x, f.position.x);
				let yroute = route(remaining_vel.y, f.position.y);

				let xroute_ex = xroute + remaining_vel.x.signum();
				let yroute_ex = yroute + remaining_vel.y.signum();

				let ychange = xroute.abs() * remaining_vel.y / remaining_vel.x.abs();
				let change = GameVec::new(xroute, ychange);

				let change_ex = change + (remaining_vel.x.signum(), 0);
				if t.check_solid(f.position + change_ex) {
					assert!(!t.check_solid(f.position + change));

					*remaining_vel -= change;
					f.position += change;

					remaining_vel.x = 0;
					f.velocity.x = reflect(f.velocity.x);
				} else {
					*remaining_vel -= change_ex;
					f.position += change_ex;
				}
			},
			Collision::Tile(XY::Y) => {
				assert!(remaining_vel.y != 0);

				let xroute = route(remaining_vel.x, f.position.x);
				let yroute = route(remaining_vel.y, f.position.y);

				let xroute_ex = xroute + remaining_vel.x.signum();
				let yroute_ex = yroute + remaining_vel.y.signum();

				let xchange = yroute.abs() * remaining_vel.x / remaining_vel.y.abs();
				let change = GameVec::new(xchange, yroute);

				let change_ex = change + (0, remaining_vel.y.signum());
				if t.check_solid(f.position + change_ex) {
					assert!(!t.check_solid(f.position + change));

					*remaining_vel -= change;
					f.position += change;

					remaining_vel.y = 0;
					f.velocity.y = reflect(f.velocity.y);
				} else {
					*remaining_vel -= change_ex;
					f.position += change_ex;
				}
			},
			Collision::Fluid { .. } => {
				unimplemented!()
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

		while let Some(coll) = self.find_next_collision(f.position, f.velocity, remaining_vel) {
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

