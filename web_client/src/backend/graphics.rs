use crate::prelude::*;
use glyph_brush::{ab_glyph, GlyphCalculatorBuilder, GlyphCalculator, Section, Text, GlyphCruncher};

const TEXT_WIDTH_SCALE: f32 = 0.0625;

pub struct WebGraphicsBackend {
	glyphs: GlyphCalculator,
}

impl WebGraphicsBackend {
	pub fn new() -> WebGraphicsBackend {
		let font = ab_glyph::FontArc::try_from_slice(include_bytes!(
			"../../../res/fonts/elementsfont.ttf"
		)).unwrap();
		let glyphs = GlyphCalculatorBuilder::using_font(font).build();

		WebGraphicsBackend {
			glyphs,
		}
	}
}

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

	// fn get_text_size(&self, text: &str, scale: f32) -> CanvasVec {
	// 	// let text = JsValue::from_serde(text).unwrap();
	// 	// let scale = JsValue::from_serde(&scale).unwrap();
	// 	// let size = js_get_text_size(text, scale);
	// 	// type V = Vec<f32>;
	// 	// size.into_serde::<V>().unwrap();
	// 	// dbg!(size);
	//
	// 	CanvasVec::new(0.0, 0.0)
	// }

	fn get_text_size(&self, text: &str, scale: f32) -> CanvasVec {
		if text.is_empty() {
			return CanvasVec::new(0.0, 0.0);
		}

		let section = Section::default()
			.add_text(Text::new(text));

		// create the scope, equivalent to a lock on the cache when
		// dropped will clean unused cached calculations like a draw call
		let mut scope = self.glyphs.cache_scope();

		let bounds = scope.glyph_bounds(section).unwrap();

		CanvasVec::new(bounds.width() * TEXT_WIDTH_SCALE, bounds.height() * TEXT_WIDTH_SCALE) * scale
	}
}
