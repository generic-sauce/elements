use crate::prelude::*;

mod canvas_vec;
mod shader;

pub use canvas_vec::*;
pub use shader::*;

pub struct DrawContext<'a> {
	pub window: &'a mut RenderWindow,
	pub texture_state: &'a TextureState,
	pub shader_state: &'a mut ShaderState,
	pub font_state: &'a FontState,
	pub animation_state: &'a AnimationState,
	pub tilemap_size: TileVec,
	pub elapsed_time: Duration,
	pub aspect_ratio: f32,
}

#[derive(PartialEq, Eq)]
pub enum Flip {
	Normal,
	Horizontal,
}

#[derive(PartialEq, Eq)]
pub enum Center {
	LeftBottom,
	LeftTop,
}

impl<'a> DrawContext<'a> {
	pub fn new(
		window: &'a mut RenderWindow,
		texture_state: &'a TextureState,
		shader_state: &'a mut ShaderState,
		font_state: &'a FontState,
		animation_state: &'a AnimationState,
		tilemap_size: TileVec,
		elapsed_time: Duration,
		aspect_ratio: f32,
		) -> DrawContext<'a>
	{
		DrawContext {
			window,
			texture_state,
			shader_state,
			font_state,
			animation_state,
			tilemap_size,
			elapsed_time,
			aspect_ratio,
		}
	}

	pub fn draw_texture(&self, position: impl IntoCanvasVec, radius: impl IntoCanvasVec, color: Color, texture: Option<&Texture>, flip: Flip) {
		let position: Vector2f = Into::<Vector2f>::into(position.to_canvas(self.tilemap_size));
		let radius: Vector2f = Into::<Vector2f>::into(radius.to_canvas(self.tilemap_size));

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

		self.window.draw_rectangle_shape(&shape, RenderStates::default());
	}

	#[allow(unused)]
	pub fn draw_sprite(&self, position: impl IntoCanvasVec, radius: impl IntoCanvasVec, color: Color, texture_id: Option<TextureId>, flip: Flip) {
		let texture = texture_id.map(|texture_id| self.texture_state.get_texture(texture_id));
		self.draw_texture(position, radius, color, texture, flip);
	}

	pub fn draw_animation(&self, position: impl IntoCanvasVec, radius: impl IntoCanvasVec, animation: Animation, flip: Flip) {
		let texture = self.animation_state.get_animation_texture(animation);
		self.draw_texture(position, radius, Color::WHITE, Some(texture), flip);
	}

	pub fn draw_text(&self, position: impl IntoCanvasVec, size: f32 /* CanvasVec coordinate system */, text: &str, center: Center) {
		let mut position = position.to_canvas(self.tilemap_size);
		let factor = self.window.size().y;
		let size = (size as f32 * factor as f32) as u32;

		// make sure factor is a multiple of size. TODO: think about this again
		let factor = ((factor / size) * size) as f32;

		let &font = &self.font_state.get_font(FontId::DefaultFont);
		let mut text = Text::new(text, &font, size);

		position.y += font.underline_position(size) / factor;
		if center == Center::LeftBottom {
			text.set_origin(Vector2f::new(0.0, text.character_size() as f32));
		}

		text.set_position(position);
		text.set_scale(Vector2f::new(1.0 / factor, -1.0 / factor));
		self.window.draw_text(&text, RenderStates::default());
	}

	pub fn draw_circle(&self, position: impl IntoCanvasVec, radius: i32 /* GameVec coordinate system */, color: Color) {
		let factor = (TILESIZE * self.tilemap_size.y) as f32;
		let position: Vector2f = Into::<Vector2f>::into(position.to_canvas(self.tilemap_size));
		let radius = radius as f32 / factor;

		let mut shape = CircleShape::new(radius, 8);
		shape.set_position(position);
		shape.set_origin(Vector2f::new(radius, radius));
		shape.set_fill_color(color);

		// shape.set_position(shape.position() * Vector2f::new(1.0, -1.0) + Vector2f::new(0.0, size.y));

		self.window.draw_circle_shape(&shape, RenderStates::default());
	}

    // NOTE: "in_texture: &mut RenderTexture" was used before!
	/*
	pub fn apply_noise(&mut self, in_texture: RenderTexture, out_target: &mut impl RenderTarget) {
		let container = TextureContainer::Render(in_texture);
		let shader = self.shader_state.get_shader(ShaderId::Noise);

		shader.set_uniform_texture("input_tex", container);

		let mut states = RenderStates::default();
		states.shader = Some(&shader.inner_shader);

		let mut rect = RectangleShape::default();
		rect.set_texture(container.texture(), true);
		rect.set_scale(Vector2f::new(1.0, -1.0));
		rect.set_size(Vector2f::new(self.aspect_ratio, -1.0));
		out_target.draw_rectangle_shape(&rect, states);
	}
	 */
}
