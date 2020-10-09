use crate::prelude::*;

mod canvas_vec;
mod shader;
mod game_draw_context;

pub use canvas_vec::*;
pub use shader::*;
pub use game_draw_context::*;

pub struct DrawContext<'a> {
	pub window_size: WindowVec,
	pub texture_state: &'a TextureState,
	pub shader_state: &'a mut ShaderState,
	pub font_state: &'a FontState,
	pub animation_state: &'a AnimationState,
	pub elapsed_time: Duration,
	pub aspect_ratio: f32,
}

#[derive(PartialEq, Eq)]
pub enum Flip {
	Normal,
	Horizontal,
}

#[derive(PartialEq, Eq)]
pub enum Origin {
	#[allow(unused)]
	Center,
	LeftBottom,
	LeftTop,
}

fn match_origin(origin: Origin, size: CanvasVec) -> CanvasVec {
	match origin {
		Origin::Center => size * 0.5,
		Origin::LeftBottom => CanvasVec::new(0.0, size.y),
		Origin::LeftTop => CanvasVec::new(0.0, 0.0),
	}
}

impl<'a> DrawContext<'a> {
	#[allow(unused)]
	pub fn draw_texture(&self, target: &impl RenderTarget, position: CanvasVec, radius: CanvasVec, color: Color, texture: Option<&Texture>, flip: Flip) {
		let position = conv(position);
		let radius = conv(radius);

		let mut shape = RectangleShape::new();
		if let Some(texture) = texture {
			shape.set_texture(texture, true);
		}
		let flipx = if flip == Flip::Horizontal { -1.0 } else { 1.0 };
		shape.set_scale(Vector2f::new(flipx, -1.0));
		shape.set_size(radius * 2.0);
		shape.set_origin(radius);
		shape.set_position(position);
		shape.set_fill_color(color);

		target.draw_rectangle_shape(&shape, RenderStates::default());
	}

	#[allow(unused)]
	pub fn draw_sprite(&self, target: &impl RenderTarget, position: CanvasVec, radius: CanvasVec, color: Color, texture_id: Option<TextureId>, flip: Flip) {
		let texture = texture_id.map(|texture_id| self.texture_state.get_texture(texture_id));
		self.draw_texture(target, position, radius, color, texture, flip);
	}

	#[allow(unused)]
	pub fn draw_rect(&self, target: &impl RenderTarget, position: CanvasVec, radius: CanvasVec, color: Color, origin: Origin) {
		let mut shape = RectangleShape::new();
		shape.set_scale(Vector2f::new(1.0, -1.0));
		shape.set_size(conv(radius * 2.0));
		shape.set_origin(conv(match_origin(origin, radius * 2.0)));
		shape.set_position(conv(position));
		shape.set_fill_color(color);

		target.draw_rectangle_shape(&shape, RenderStates::default());
	}

	#[allow(unused)]
	pub fn draw_animation(&self, target: &impl RenderTarget, position: CanvasVec, radius: CanvasVec, animation: Animation, flip: Flip) {
		let texture = self.animation_state.get_animation_texture(animation);
		self.draw_texture(target, position, radius, Color::WHITE, Some(texture), flip);
	}

	#[allow(unused)]
	pub fn draw_text(&self, target: &impl RenderTarget, mut position: CanvasVec, size: f32 /* CanvasVec coordinate system */, text: &str, origin: Origin) {
		let factor = self.window_size.y as u32;
		let size = (size as f32 * factor as f32) as u32;

		// make sure factor is a multiple of size. TODO: think about this again
		let factor = ((factor / size) * size) as f32;

		let &font = &self.font_state.get_font(FontId::DefaultFont);
		let mut text = Text::new(text, &font, size);

		position.y += font.underline_position(size) / factor;
		if origin == Origin::LeftBottom {
			text.set_origin(Vector2f::new(0.0, text.character_size() as f32));
		}

		text.set_position(conv(position));
		text.set_scale(Vector2f::new(1.0 / factor, -1.0 / factor));
		target.draw_text(&text, RenderStates::default());
	}

	#[allow(unused)]
	pub fn get_text_width(&self, size: f32, text: &str) -> f32 {
		let factor = self.window_size.y as u32;
		let size = (size as f32 * factor as f32) as u32;
		let factor = ((factor / size) * size) as f32;
		let &font = &self.font_state.get_font(FontId::DefaultFont);
		let mut text = Text::new(text, &font, size);
		text.set_scale(Vector2f::new(1.0 / factor, -1.0 / factor));
		text.global_bounds().width

	}

	#[allow(unused)]
	pub fn draw_circle(&self, target: &impl RenderTarget, position: CanvasVec, radius: f32 /* GameVec coordinate system */, color: Color) {
		let position: Vector2f = conv(position);

		let mut shape = CircleShape::new(radius, 8);
		shape.set_position(position);
		shape.set_origin(Vector2f::new(radius, radius));
		shape.set_fill_color(color);

		// shape.set_position(shape.position() * Vector2f::new(1.0, -1.0) + Vector2f::new(0.0, size.y));

		target.draw_circle_shape(&shape, RenderStates::default());
	}

	// NOTE: "in_texture: &mut RenderTexture" was used before!
	#[allow(unused)]
	pub fn apply_noise(&mut self, target: &impl RenderTarget, texture: RenderTexture) {
		let container = TextureContainer::Render(texture);
		let shader = self.shader_state.get_shader(ShaderId::Noise);

		shader.set_uniform_texture("input_tex", container);

		let mut states = RenderStates::default();
		states.shader = Some(&shader.inner_shader);

		let mut rect = RectangleShape::default();
		rect.set_texture(self.texture_state.get_texture(TextureId::Any), true);
		rect.set_size(Vector2f::new(self.aspect_ratio, 1.0));

		target.draw_rectangle_shape(&rect, states);
	}

	#[allow(unused)]
	pub fn fill_canvas_with_texture(&mut self, target: &impl RenderTarget, texture: RenderTexture) {
		let mut rect = RectangleShape::default();
		rect.set_texture(texture.texture(), true);
		rect.set_size(Vector2f::new(self.aspect_ratio, 1.0));

		target.draw_rectangle_shape(&rect, RenderStates::default());
	}

	#[allow(unused)]
	pub fn fill_canvas_with_states(&self, target: &impl RenderTarget, states: RenderStates) {
		let mut rect = RectangleShape::default();
		rect.set_texture(self.texture_state.get_texture(TextureId::Any), true);
		rect.set_scale(Vector2f::new(1.0, -1.0));
		rect.set_size(Vector2f::new(self.aspect_ratio, -1.0));

		target.draw_rectangle_shape(&rect, states);
	}

	#[allow(unused)]
	pub fn fill_canvas_with_color(&self, target: &impl RenderTarget, color: Color) {
		let mut rect = RectangleShape::default();
		rect.set_fill_color(color);
		rect.set_size(Vector2f::new(self.aspect_ratio, 1.0));

		target.draw_rectangle_shape(&rect, RenderStates::default());
	}
}

fn conv(v: CanvasVec) -> Vector2f {
	Vector2f::new(v.x, v.y)
}
