use crate::prelude::*;

pub struct GameDrawContext<'a> {
	pub ctxt: DrawContext<'a>,
	pub tilemap_size: TileVec,
	pub tilemap_texture: &'a Texture,
}

impl<'a> GameDrawContext<'a> {
	pub fn draw_texture(&self, target: &impl RenderTarget, position: impl IntoCanvasVec, radius: impl IntoCanvasVec, color: Color, texture: Option<&Texture>, flip: Flip) {
		let position = position.to_canvas(self.tilemap_size);
		let radius = radius.to_canvas(self.tilemap_size);
		self.ctxt.draw_texture(target, position, radius, color, texture, flip);
	}

	pub fn draw_sprite(&self, target: &impl RenderTarget, position: impl IntoCanvasVec, radius: impl IntoCanvasVec, color: Color, texture_id: Option<TextureId>, flip: Flip) {
		let position = position.to_canvas(self.tilemap_size);
		let radius = radius.to_canvas(self.tilemap_size);
		self.ctxt.draw_sprite(target, position, radius, color, texture_id, flip);
	}

	pub fn draw_rect(&self, target: &impl RenderTarget, position: impl IntoCanvasVec, radius: impl IntoCanvasVec, color: Color, origin: Origin) {
		let position = position.to_canvas(self.tilemap_size);
		let radius = radius.to_canvas(self.tilemap_size);
		self.ctxt.draw_rect(target, position, radius, color, origin);
	}

	pub fn draw_animation(&self, target: &impl RenderTarget, position: impl IntoCanvasVec, radius: impl IntoCanvasVec, animation: Animation, flip: Flip) {
		let position = position.to_canvas(self.tilemap_size);
		let radius = radius.to_canvas(self.tilemap_size);
		self.ctxt.draw_animation(target, position, radius, animation, flip);
	}

	pub fn draw_text(&self, target: &impl RenderTarget, position: impl IntoCanvasVec, size: f32 /* CanvasVec coordinate system */, text: &str, origin: Origin) {
		let position = position.to_canvas(self.tilemap_size);
		self.ctxt.draw_text(target, position, size, text, origin);
	}

	pub fn draw_circle(&self, target: &impl RenderTarget, position: impl IntoCanvasVec, radius: i32 /* GameVec coordinate system */, color: Color) {
		let position = position.to_canvas(self.tilemap_size);
		let factor = (TILESIZE * self.tilemap_size.y) as f32;
		let radius = radius as f32 / factor;
		self.ctxt.draw_circle(target, position, radius, color);
	}
}
