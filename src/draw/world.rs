use super::*;

const TROPHY_SIZE: CanvasVec = CanvasVec::new(0.05, 0.05);
const TROPHY_SHOW_START: u32 = 100;
const WINNER_POSITIONS: [ViewVec; 2] = [ViewVec::new(0.0, 0.9), ViewVec::new(1.0, 0.9)];

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
		draw_trophy(draw, self);

		let text_size = 0.04;
		draw.text(ViewVec::new(0.0, 1.0 - 1.0 * text_size), text_size, Color::WHITE,
			&*format!("best of {}", self.best_of_n));
		draw.text(ViewVec::new(0.0, 1.0 - 2.0 * text_size), text_size, Color::WHITE,
			&*format!("score: blue {} / red {}", self.kills[0], self.kills[1]));
	}
}

fn draw_trophy(draw: &mut Draw, world: &World) {
	let trophy_start_position: ViewVec = world.tilemap.size.to_game().to_view() / 2.0;
	let trophy_size = TROPHY_SIZE.to_view();
	if let RestartState::Restart { winner, counter, .. } = world.restart_state {
		let trophy_position_mix: f32 = ((counter as f32 - RESTART_DELAY_COUNT as f32) / (TROPHY_END_COUNT as f32 - RESTART_DELAY_COUNT as f32)).min(1.0).max(0.0);
		match winner {
			Winner::None => {
				if counter >= TROPHY_SHOW_START {
					draw.texture(trophy_start_position - trophy_size, trophy_start_position + trophy_size, TextureId::Trophy, Flip::Normal, None);
				}
			}
			Winner::One(winner) => {
				let pos = trophy_start_position.mix(WINNER_POSITIONS[winner as usize], 1.0 - trophy_position_mix, trophy_position_mix);
				draw.texture(pos - trophy_size, pos + trophy_size, TextureId::Trophy, Flip::Normal, None);
			}
			Winner::Both => {
				let pos = trophy_start_position.mix(WINNER_POSITIONS[0], 1.0 - trophy_position_mix, trophy_position_mix);;
				draw.texture(pos - trophy_size, pos + trophy_size, TextureId::Trophy, Flip::Normal, None);
				let pos = trophy_start_position.mix(WINNER_POSITIONS[1], 1.0 - trophy_position_mix, trophy_position_mix);;
				draw.texture(pos - trophy_size, pos + trophy_size, TextureId::Trophy, Flip::Normal, None);
			}
		}
	}
}
