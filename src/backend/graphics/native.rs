use crate::prelude::*;
use wgpu_glyph::{ab_glyph, Section, Text, GlyphCruncher};
use glyph_brush::{GlyphCalculatorBuilder, GlyphCalculator};

const TEXT_WIDTH_SCALE: f32 = 0.0625;

pub struct NativeGraphicsBackend {
	pub draw_sender: Sender<Draw>,
	glyphs: GlyphCalculator,
}

impl NativeGraphicsBackend {
	pub fn new(draw_sender: Sender<Draw>) -> NativeGraphicsBackend {
		let font = ab_glyph::FontArc::try_from_slice(include_bytes!(
			"../../../res/fonts/dashing_unicorn.ttf"
		)).unwrap();
		let glyphs = GlyphCalculatorBuilder::using_font(font).build();

		NativeGraphicsBackend {
			draw_sender,
			glyphs,
		}
	}
}

impl GraphicsBackend for NativeGraphicsBackend {
	fn draw(&mut self, draw: Draw, _: Option<&World>) {
		self.draw_sender.send(draw).unwrap();
	}

	fn get_text_width(&self, text: &str) -> CanvasVec {
		if text.is_empty() {
			return CanvasVec::new(0.0, 0.0);
		}

		let section = Section::default()
			.add_text(Text::new(text));

		// create the scope, equivalent to a lock on the cache when
		// dropped will clean unused cached calculations like a draw call
		let mut scope = self.glyphs.cache_scope();

		let bounds = scope.glyph_bounds(section).unwrap();

		CanvasVec::new(bounds.width() * TEXT_WIDTH_SCALE, bounds.height() * TEXT_WIDTH_SCALE)
	}
}
