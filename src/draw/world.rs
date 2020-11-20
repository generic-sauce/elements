use super::*;

const TROPHY_SIZE: CanvasVec = CanvasVec::new(0.035, 0.035);
const TROPHY_SHOW_START: u32 = 115;
const WINNER_POSITIONS: [ViewVec; 2] = [ViewVec::new(0.065, 0.95), ViewVec::new(0.97, 0.95)];

impl World {
	pub fn draw<B: Backend>(&self, draw: &mut Draw, app: &App<B>) {
		let sky_color = Color::rgb(
			121.0 / 255.0,
			181.0 / 255.0,
			235.0 / 255.0,
		);

		let sky_color = match self.restart_state {
			RestartState::Restart { counter, .. } => {
				let rdc = FIGHT_END_COUNT as f32;
				let counter = counter as f32;
				let factor = (rdc - counter.min(rdc) * 0.9) / rdc;
				sky_color * 0.8 * factor
			},
			RestartState::Game => sky_color
		};

		draw.set_clear_color(sky_color);
		draw.map(&self.tilemap, &self.fluidmap);
		draw_players(draw, self);
		draw_cursors(draw, self);
		draw_healthbars(draw, self);
		draw_trophy(draw, self, app);
	}
}

fn trophy_position_curve(mix: f32) -> f32 {
	f32::sin(0.5*mix*std::f32::consts::PI).powf(3.0)
}

fn draw_trophy<B: Backend>(draw: &mut Draw, world: &World, app: &App<B>) {
	// winner trophy
	let trophy_start_position: ViewVec = world.tilemap.size.to_game().to_view() / 2.0;
	let trophy_size = TROPHY_SIZE.to_view();
	if let RestartState::Restart { winner, counter, .. } = world.restart_state {
		let trophy_position_mix: f32 = ((counter as f32 - FIGHT_END_COUNT as f32) / (TROPHY_END_COUNT as f32 - FIGHT_END_COUNT as f32)).min(1.0).max(0.0);
		let trophy_position_mix = trophy_position_curve(trophy_position_mix);
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
				let pos = trophy_start_position.mix(WINNER_POSITIONS[0], 1.0 - trophy_position_mix, trophy_position_mix);
				draw.texture(pos - trophy_size, pos + trophy_size, TextureId::Trophy, Flip::Normal, None);
				let pos = trophy_start_position.mix(WINNER_POSITIONS[1], 1.0 - trophy_position_mix, trophy_position_mix);
				draw.texture(pos - trophy_size, pos + trophy_size, TextureId::Trophy, Flip::Normal, None);
			}
		}
	}

	// status trophies
	let text_size = 0.037;
	for i in 0..2 {
		let text = format!("{} x", world.kills[i]);
		let text_offset = app.graphics_backend.get_text_size(&text, text_size).x * 0.5 + 0.02;
		draw.text(WINNER_POSITIONS[i] - ViewVec::new(text_offset, 0.03), text_size, Color::WHITE, &text);
		draw.texture(WINNER_POSITIONS[i] - trophy_size, WINNER_POSITIONS[i] + trophy_size, TextureId::Trophy, Flip::Normal, None);
	}
}
