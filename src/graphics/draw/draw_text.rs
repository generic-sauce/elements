use crate::prelude::*;
use crate::graphics::*;
use wgpu_glyph::{ab_glyph, GlyphBrush, GlyphBrushBuilder, Section, Text};

pub(in crate::graphics) struct DrawText {
	glyph_brush: GlyphBrush<(), ab_glyph::FontArc>,
	staging_belt: wgpu::util::StagingBelt,
}

impl DrawText {
	pub(in crate::graphics) fn new(device: &wgpu::Device) -> DrawText {
		let font = ab_glyph::FontArc::try_from_slice(include_bytes!(
			"../../../res/fonts/dashing_unicorn.ttf"
		)).unwrap();

		let glyph_brush = GlyphBrushBuilder::using_font(font)
			.build(&device, SURFACE_FORMAT);

		let staging_belt = wgpu::util::StagingBelt::new(1024);

		DrawText {
			glyph_brush,
			staging_belt,
		}
	}

	pub(in crate::graphics) fn render(
		&mut self,
		context: &mut GraphicsContext,
		draw: &Draw,
	) {
		futures::executor::block_on(self.staging_belt.recall());

		for text in &draw.texts {
			let left_bot = text.left_bot.to_f() * context.window_size.to_f();
			let left_bot = (left_bot.x, left_bot.y);
			let color = [text.color.r, text.color.g, text.color.b, text.color.a];

			self.glyph_brush.queue(Section {
				screen_position: left_bot,
				bounds: (context.window_size.x as f32, context.window_size.y as f32),
				text: vec![
					Text::new(&*text.string)
						.with_color(color)
						.with_scale(text.scale),
				],
				..Section::default()
			});
		}

		// Draw the text!
		self.glyph_brush
			.draw_queued(
				context.device,
				&mut self.staging_belt,
				context.encoder,
				&context.swap_chain_texture.view,
				context.window_size.x,
				context.window_size.y,
			)
			.expect("Draw queued");

		self.staging_belt.finish();
	}
}
