use super::*;

pub struct GraphicsWorld {
	pub tilemap_size: TileVec,
	pub tilemap_data: Vec<u8>,
	pub fluidmap_data: Vec<u8>,
	pub depth_index: DepthIndex,
}

impl GraphicsWorld {
	pub fn new(tilemap: &TileMap, fluidmap: &FluidMap, depth_index: DepthIndex) -> GraphicsWorld {

		let tilemap_data: Vec<u8> = tilemap.iter()
			.map(|p| tilemap.get(p))
			.map(|t| match t {
				Tile::Void => 0,
				Tile::Ground => 1,
				Tile::Wall { owner, .. } => 2 + owner as u8,
			})
			.collect();

		let mut fluidmap_data: Vec<u8> = Vec::new();
		fluidmap_data.resize((4 * tilemap.size.x * tilemap.size.y) as usize, 0 as u8);

		for fluid in fluidmap.iter() {
			let cell_id = fluid.position / TILESIZE;
			let local_position = ((fluid.position.x % TILESIZE) as u8, (fluid.position.y % TILESIZE) as u8);

			let cell_index = 4 * (cell_id.x + cell_id.y * tilemap.size.x as i32) as usize;
			fluidmap_data[cell_index+3] = 255;
			fluidmap_data[cell_index+2] = (fluid.owner * 255) as u8;
			fluidmap_data[cell_index+1] = local_position.1 as u8;
			fluidmap_data[cell_index] = local_position.0 as u8;
		}

		GraphicsWorld {
			tilemap_size: tilemap.size,
			tilemap_data,
			fluidmap_data,
			depth_index,
		}
	}
}

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
		draw.world(&self.tilemap, &self.fluidmap);
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
