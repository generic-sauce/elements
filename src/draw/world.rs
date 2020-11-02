use crate::prelude::*;
use super::*;

pub struct GraphicsWorld {
	pub tilemap_size: TileVec,
	pub tilemap_data: Vec<u8>,
	pub fluidmap_data: Vec<u8>,
}

impl GraphicsWorld {
	pub fn new(tilemap: &TileMap, fluidmap: &FluidMap) -> GraphicsWorld {

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
		}
	}
}

impl<B: Backend> ClientWorld<B> {
	pub fn draw(&mut self, draw: &mut Draw) {
		draw.world(&self.world.tilemap, &self.world.fluidmap);
		draw_players(draw, &self.world);
		draw_cursors(draw, &self.world);
		draw_healthbars(draw, &self.world);
		draw.text(v(0.0, 0.0), 40.0, Color::WHITE, "Hello Text!");
	}
}
