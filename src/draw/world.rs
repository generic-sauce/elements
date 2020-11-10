use super::*;

impl World {
	pub fn draw(&self, draw: &mut Draw) {
		let sky_color = match self.restart_state {
			RestartState::Restart { counter, .. } => {
				let rdc = RESTART_DELAY_COUNT as f32;
				let counter = counter as f32;
				let factor = (rdc - counter.min(rdc) * 0.9) / rdc;
				Color::rgb(
					50.0 / 255.0,
					120.0 / 255.0,
					215.0 / 255.0,
				) * 0.8 * factor
			},
			RestartState::Game => {
				Color::rgb(
					50.0 / 255.0,
					120.0 / 255.0,
					215.0 / 255.0,
				)
			}
		};

		draw.set_clear_color(sky_color);
		draw.map(&self.tilemap, &self.fluidmap);
		draw_players(draw, self);
		draw_cursors(draw, self);
		draw_healthbars(draw, self);

		let text_size = 0.04;
		draw.text(ViewVec::new(0.0, 1.0 - 1.0 * text_size), text_size, Color::WHITE,
			&*format!("best of {}", self.best_of_n));
		draw.text(ViewVec::new(0.0, 1.0 - 2.0 * text_size), text_size, Color::WHITE,
			&*format!("score: blue {} / red {}", self.kills[0], self.kills[1]));
	}
}
