use super::*;
use crate::prelude::*;

pub fn draw_lobby<B: Backend>(draw: &mut Draw, graphics_backend: &dyn GraphicsBackend) {
	draw.set_clear_color(Color::gray(0.02));

	let t = B::now() as f32 / 1000.0;
	let arc_offset = f32::powf(f32::abs(f32::sin(t)), 0.9) * f32::signum(f32::sin(t));
	draw.arc(ViewVec::new(0.5, 0.5), 0.1, Color::WHITE, 0.5, arc_offset);
	draw.circle(ViewVec::new(0.5, 0.5), 0.08, Color::gray(0.3));

	let text = "Waiting for more players...";
	let scale = 0.07;
	let size = graphics_backend.get_text_size(text, scale);
	draw.text(CanvasVec::center(0.0, 0.3) - size * 0.5, scale, Color::WHITE, text);
}
