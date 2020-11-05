use crate::prelude::*;

pub struct WebGraphicsBackend;

impl GraphicsBackend for WebGraphicsBackend {
	fn draw(&mut self, _draw: Draw, world: Option<&World>) {
		if let Some(w) = world {
			RenderWorld::draw(w);
		}
	}
}
