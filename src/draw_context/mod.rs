use crate::prelude::*;

mod canvas_vec;

pub use canvas_vec::*;

pub struct DrawContext<'a> {
	pub window: &'a mut RenderWindow,
	pub texture_state: &'a TextureState,
	pub shader_state: &'a mut ShaderState,
	pub font_state: &'a FontState,
	pub animation_state: &'a AnimationState,
	pub tilemap_size: TileVec,
	pub elapsed_time: Duration,
}

#[derive(PartialEq, Eq)]
pub enum Flip {
	Normal,
	Horizontal,
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
		}
	}

	pub fn draw_texture(&self, position: impl IntoCanvasVec, radius: impl IntoCanvasVec, color: Color, texture: Option<&Texture>, flip: Flip) {
		let size = Vector2f::new(self.window.size().x as f32, self.window.size().y as f32);
		let position: Vector2f = Into::<Vector2f>::into(position.to_canvas(self.tilemap_size)) * size.y;
		let radius: Vector2f = Into::<Vector2f>::into(radius.to_canvas(self.tilemap_size)) * size.y;

		let mut shape = RectangleShape::new();
		if let Some(texture) = texture {
			shape.set_texture(texture, true);
		}
		let flipx = if flip == Flip::Horizontal { -1.0 } else { 1.0 };
		shape.set_scale(Vector2f::new(flipx, 1.0));
		shape.set_size(radius * 2.0);
		shape.set_origin(radius);
		shape.set_position(position);
		shape.set_fill_color(color);
		shape.set_position(shape.position() * Vector2f::new(1.0, -1.0) + Vector2f::new(0.0, size.y));

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

	pub fn draw_text(&self, position: impl IntoCanvasVec, size: u32, text: &str) {
		let position = position.to_canvas(self.tilemap_size);

		let font = self.font_state.get_font(FontId::DefaultFont);
		let mut text = Text::new(text, &font, size);
		let center = Vector2f::new(0.0, text.local_bounds().height + font.underline_position(size));
		text.set_position(center);
		// text.set_position(Vector2f::new(100.0, 100.0));
		text.set_position(position * (self.window.size().y as f32) + center);

		let window_height = self.window.size().y as f32;
		text.set_position(text.position() * Vector2f::new(1.0, -1.0) + Vector2f::new(0.0, window_height));
		self.window.draw_text(&text, RenderStates::default());
	}

	pub fn draw_circle(&self, position: impl IntoCanvasVec, radius: i32 /* GameVec coordinate system */, color: Color) {
		let size = Vector2f::new(self.window.size().x as f32, self.window.size().y as f32);
		let position: Vector2f = Into::<Vector2f>::into(position.to_canvas(self.tilemap_size)) * size.y;
		let radius = radius as f32 * size.y / (TILESIZE * self.tilemap_size.y) as f32;

		let mut shape = CircleShape::new(radius, 32);
		shape.set_position(position);
		shape.set_origin(Vector2f::new(radius, radius));
		shape.set_fill_color(color);

		shape.set_position(shape.position() * Vector2f::new(1.0, -1.0) + Vector2f::new(0.0, size.y));

		self.window.draw_circle_shape(&shape, RenderStates::default());
	}
}
