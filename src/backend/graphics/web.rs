use crate::prelude::*;

pub struct WebGraphicsBackend;

impl GraphicsBackend for WebGraphicsBackend {
	fn submit(&mut self, draw: Draw) {
		let draw = WebDraw::new(draw);

		js_render(
			draw.json_draw,
			draw.tilemap_data,
			draw.fluidmap_data,
		)
	}

	fn get_text_size(&self, _text: &str, _scale: f32) -> CanvasVec {
		unimplemented!()
	}
}
