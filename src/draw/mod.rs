mod player;
use player::*;
mod canvas_vec;
pub use canvas_vec::*;

#[derive(Copy, Clone)]
pub struct Color {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32,
}

impl Color {
	pub const fn rgb(r: f32, g: f32, b: f32) -> Color {
		Color {
			r, g, b, a: 1.0,
		}
	}

	pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
		Color {
			r, g, b, a,
		}
	}

	pub const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
	pub const GRAY: Color = Color::rgb(0.2, 0.2, 0.2);
	pub const BLACK: Color = Color::rgb(0.0, 0.0, 0.0);
	pub const RED: Color = Color::rgb(1.0, 0.0, 0.0);
	pub const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);
	pub const BLUE: Color = Color::rgb(0.0, 0.0, 1.0);
}

use crate::prelude::*;

impl<B: Backend> ClientWorld<B> {
	pub fn draw(&mut self, draw: &mut Draw) {
		draw.world(&self.world.tilemap, &self.world.fluidmap);
		draw_players(draw, &self.world);
		draw_cursors(draw, &self.world);
		draw_healthbars(draw, &self.world);
	}
}
