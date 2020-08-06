use crate::prelude::*;

pub struct Sensor {
	pub left_bot: Vec2i,
	pub size: Vec2i,
}

impl Player {
	pub(in super) fn check_sensor(&self, s: &'static Sensor, t: &TileMap) -> bool {
		let lb = self.left_bot + s.left_bot;
		let x_min = lb.x / TILESIZE;
		let x_max = (lb.x + s.size.x - 1) / TILESIZE; // recall that size.x = 1, means that x_min = x_max

		let y_min = lb.y / TILESIZE;
		let y_max = (lb.y + s.size.y - 1) / TILESIZE;

		// TODO write this using .any()?
		for x in x_min..=x_max {
			for y in y_min..=y_max {
				if t.get((x as u32, y as u32).into()).is_solid() { return true; }
			}
		}
		false
	}
}
