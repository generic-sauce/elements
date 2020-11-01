mod player;
use player::*;

mod fluidmap;
mod tilemap;
mod hud;
// mod render;
// pub use render::*;

use crate::prelude::*;

impl<B: Backend> ClientWorld<B> {
	pub fn draw(&mut self, draw: &mut Draw) {
		draw.world(&self.world.tilemap, &self.world.fluidmap);
		draw_players(draw, &self.world);
		draw_cursors(draw, &self.world);
		draw_healthbars(draw, &self.world);
	}
}
