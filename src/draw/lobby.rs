use super::*;

pub fn draw_lobby(draw: &mut Draw, _graphics_backend: &dyn GraphicsBackend, elapsed_ms: f32) {
	draw.set_clear_color(Color::gray(0.02));

	let t = elapsed_ms / 1000.0;
	let arc_offset = f32::powf(f32::abs(f32::sin(t)), 0.9) * f32::signum(f32::sin(t));
	draw.arc(ViewVec::new(0.5, 0.5), 0.1, Color::WHITE, 0.5, arc_offset);
	draw.circle(ViewVec::new(0.5, 0.5), 0.08, Color::gray(0.3));

	draw.text(ViewVec::new(0.5, 0.8), 0.07, Color::WHITE, "Waiting for more players...")
}
