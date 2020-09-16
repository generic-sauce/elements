use crate::prelude::*;

/*
enum XY {
	X,
	Y,
}

enum Collision {
	Fluid { me: usize, other: usize, },
	Tile(XY),
}

impl Fluid {
	pub(in super) fn move_and_slide(&mut self, mut remaining_vel: GameVec, t: &TileMap) {
}

fn find_next_collision(fm: &FluidMap, pos: GameVec, velocity: GameVec, remaining_vel: GameVec) -> Option<Collision> {
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
}

// returns the change required to touch, but not collide the next tile
fn route(velocity: i32, pos: i32) -> i32 {
	if velocity < 0 {
		-(pos % TILESIZE)
	} else {
		(TILESIZE-1) - (pos % TILESIZE)
	}
}

impl FluidMap {
	pub(in super) fn tick_physics(&mut self, t: &TileMap) {
		let mut grid = Vec::new();
		std::mem::swap(&mut grid, &mut self.grid);

		let mut fluids: Vec<Fluid> = grid.into_iter()
			.map(|x| x.into_iter())
			.flatten()
			.collect();

		let n = fluids.len();
		for i in 0..n {
			move_fluid(&mut fluids[..], i);
		}

		self.grid = FluidMap::mk_grid(fluids.into_iter(), self.size);
	}
}

fn move_fluid(fluids: &mut [Fluid], i: usize, t: &TileMap) {
	if t.check_solid(fluids[i].position) {
		#[cfg(debug_assertions)]
		println!("A fluid is glitched.");
		return;
	}

	let mut remaining_vel = fluids[i].velocity;

	let reflect = |x: i32| -x / 3; // TODO add some randomness.

	while remaining_vel != 0.into() {
		let xroute = route(remaining_vel.x, fluids[i].position.x);
		let yroute = route(remaining_vel.y, fluids[i].position.y);

		let xroute_ex = xroute + remaining_vel.x.signum();
		let yroute_ex = yroute + remaining_vel.y.signum();

		if xroute.abs() >= remaining_vel.x.abs() && yroute.abs() >= remaining_vel.y.abs() { // if no more collisions can happen!
			fluids[i].position += remaining_vel;
			break;
		} else if (remaining_vel.y == 0 && yroute_ex == 0) /* edge case */ || (xroute_ex * remaining_vel.y).abs() < (yroute_ex * remaining_vel.x).abs() { //    <->    xroute / self.velocity.x < yroute / self.velocity.y    <->    xtime < ytime
			assert!(remaining_vel.x != 0);

			let ychange = xroute.abs() * remaining_vel.y / remaining_vel.x.abs();
			let change = GameVec::new(xroute, ychange);

			let change_ex = change + (remaining_vel.x.signum(), 0);
			if t.check_solid(fluids[i].position + change_ex) {
				assert!(!t.check_solid(fluids[i].position + change));

				remaining_vel -= change;
				fluids[i].position += change;

				remaining_vel.x = 0;
				fluids[i].velocity.x = reflect(fluids[i].velocity.x);
			} else {
				remaining_vel -= change_ex;
				fluids[i].position += change_ex;
			}
		} else {
			assert!(remaining_vel.y != 0);

			let xchange = yroute.abs() * remaining_vel.x / remaining_vel.y.abs();
			let change = GameVec::new(xchange, yroute);

			let change_ex = change + (0, remaining_vel.y.signum());
			if t.check_solid(fluids[i].position + change_ex) {
				assert!(!t.check_solid(fluids[i].position + change));

				remaining_vel -= change;
				fluids[i].position += change;

				remaining_vel.y = 0;
				fluids[i].velocity.y = reflect(fluids[i].velocity.y);
			} else {
				remaining_vel -= change_ex;
				fluids[i].position += change_ex;
			}
		}
	}
}
*/

impl FluidMap {
	pub(in super) fn tick_physics(&mut self, t: &TileMap) {
		// TODO
	}
}
