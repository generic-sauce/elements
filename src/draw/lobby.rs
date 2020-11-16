use super::*;

pub fn draw_lobby(draw: &mut Draw, elapsed_ms: f32) {
	draw.set_clear_color(Color::gray(0.02));

	let arc_offset = f32::sin(elapsed_ms / 2000.0);
	draw.arc(ViewVec::new(0.5, 0.5), 0.1, Color::WHITE, 0.5, arc_offset);
	draw.circle(ViewVec::new(0.5, 0.5), 0.08, Color::gray(0.3));

	draw.text(ViewVec::new(0.5, 0.8), 0.07, Color::WHITE, "Waiting for more players...")
}
