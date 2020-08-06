use crate::prelude::*;

impl TileMap {
	pub fn draw(&mut self, context: &mut DrawContext) {
		context.draw_tilemap(self);
	}
}
