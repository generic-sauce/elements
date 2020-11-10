use crate::graphics::*;
use wgpu_glyph::{ab_glyph, GlyphBrush, GlyphBrushBuilder, Section, Text};

pub(in crate::graphics) struct RenderText {
	glyph_brush: GlyphBrush<(), ab_glyph::FontArc>,
	staging_belt: wgpu::util::StagingBelt,
}

impl RenderText {
	pub(in crate::graphics) fn new(device: &wgpu::Device) -> RenderText {
		let font = ab_glyph::FontArc::try_from_slice(include_bytes!(
			"../../../res/fonts/elementsfont.ttf"
		)).unwrap();

		let glyph_brush = GlyphBrushBuilder::using_font(font)
			.build(&device, SURFACE_FORMAT);

		let staging_belt = wgpu::util::StagingBelt::new(1024);

		RenderText {
			glyph_brush,
			staging_belt,
		}
	}

	pub(in crate::graphics) fn render(
		&mut self,
		context: &mut GraphicsContext,
		draw: &RenderDraw,
	) {
		self.staging_belt.recall();

		for text in &draw.texts {
			let window_size = context.window_size.to_subpixel();

			let mut left_bot = text.left_bot;
			left_bot.y = 1.0 - left_bot.y;
			left_bot.y -= text.scale;
			let left_bot = left_bot.to_subpixel(window_size);
			let left_bot = (left_bot.x, left_bot.y);

			let scale = window_size.y * f32::min(1.0, window_view_ratio(window_size));
			let scale = text.scale * scale;

			let color = text.color;
			let color = [color.r, color.g, color.b, color.a];

			self.glyph_brush.queue(Section {
				screen_position: left_bot,
				bounds: (window_size.x, window_size.y),
				text: vec![
					Text::new(&*text.string)
						.with_color(color)
						.with_scale(scale),
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
				context.window_size.x as u32,
				context.window_size.y as u32,
			)
			.expect("Draw queued");

		self.staging_belt.finish();
	}
}
