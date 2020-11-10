use crate::prelude::*;

pub struct WebGraphicsBackend;

impl GraphicsBackend for WebGraphicsBackend {
	fn submit(&mut self, draw: Draw) {
		let draw = WebRenderDraw::new(draw);

		js_render(
			draw.js_web_render_draw,
			draw.tilemap_data,
			draw.fluidmap_data,
			draw.vertex_data,
		)
	}

	fn get_text_size(&self, _text: &str, _scale: f32) -> CanvasVec {
		unimplemented!()
	}
}
