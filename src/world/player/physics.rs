use crate::prelude::*;

impl Player {
	pub(in super) fn move_by_velocity(&mut self, t: &TileMap) {
		let mut remaining_vel = self.velocity;

		while remaining_vel != 0.into() {
			let xroute = route(remaining_vel.x, self.left_bot.x, PLAYER_SIZE.x);
			let yroute = route(remaining_vel.y, self.left_bot.y, PLAYER_SIZE.y);

			assert!(xroute != 0);
			assert!(yroute != 0);

			if xroute.abs() > remaining_vel.x.abs() && yroute.abs() > remaining_vel.y.abs() { // if no more collisions can happen!
				self.left_bot += remaining_vel;
				break;
			} else if (xroute * self.velocity.y).abs() < (yroute * self.velocity.x).abs() { //    <->    xdist / self.velocity.x < ydist / self.velocity.y    <->    xtime < ytime
				assert!(false);

				let ychange = xroute.abs() * remaining_vel.y / remaining_vel.x.abs();
				let change = Vec2i::new(xroute, ychange);

				remaining_vel -= change;
				self.left_bot += change;

				// TODO this should only happen upon collision!
				// remaining_vel.x = 0;
				// self.velocity.x = 0;
			} else {
				let xchange = yroute.abs() * remaining_vel.x / remaining_vel.y.abs();
				let change = Vec2i::new(xchange, yroute);

				assert!(change != 0.into());

				remaining_vel -= change;
				self.left_bot += change;

				// remaining_vel.y = 0;
				// self.velocity.y = 0;
			}
		}
	}
}

fn is_colliding(left_bot: Vec2i, t: &TileMap) -> bool {
	let x_min = left_bot.x / TILESIZE;
	let x_max = (left_bot.x + PLAYER_SIZE.x - 1) / TILESIZE; // recall that PLAYER_SIZE = 1, means that x_min = x_max

	let y_min = left_bot.y / TILESIZE;
	let y_max = (left_bot.y + PLAYER_SIZE.y - 1) / TILESIZE;

	// TODO write this using .any()?
	for x in x_min..=x_max {
		for y in y_min..=y_max {
			if t.get((x as u32, y as u32).into()).is_solid() { return true; }
		}
	}
	false
}

fn route(velocity: i32, min: i32, size: i32) -> i32 {
	if velocity < 0 {
		-(min % TILESIZE) - 1
	} else {
		let max = min + size - 1;
		(max / TILESIZE) + 1
	}
}
