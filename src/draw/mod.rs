mod player;
mod fluidmap;
mod tilemap;
mod hud;
// mod render;
// pub use render::*;

use crate::prelude::*;

impl<B: Backend> ClientWorld<B> {
	pub fn draw(&mut self, app: &mut App<B>, timed_loop_info: &TimedLoopInfo) {
	}
}
