use crate::prelude::*;

pub trait GraphicsBackend {
	fn submit(&mut self, draw: Draw);
	fn get_text_size(&self, text: &str, scale: f32) -> CanvasVec;
}
