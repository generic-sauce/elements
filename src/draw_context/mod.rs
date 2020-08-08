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

	pub fn draw_texture<T: IntoCanvasVec>(&self, position: T, radius: T, color: Color, texture: Option<&Texture>) {
		let size = Vector2f::new(self.window.size().x as f32, self.window.size().y as f32);
		let position: Vector2f = Into::<Vector2f>::into(position.to_canvas(self.tilemap_size)) * size.y;
		let radius: Vector2f = Into::<Vector2f>::into(radius.to_canvas(self.tilemap_size)) * size.y;

		let mut shape = RectangleShape::new();
		if let Some(texture) = texture {
			shape.set_texture(texture, true);
		}
		shape.set_size(radius * 2.0);
		shape.set_origin(radius);
		shape.set_position(position);
		shape.set_fill_color(color);

		// let ratio = size.x / size.y;
		// let height = self.tilemap_size.y as f32;
		// let tile = size.y / height;
		// shape.set_scale(Vector2f::new(1.0, -1.0));
		shape.set_position(shape.position() * Vector2f::new(1.0, -1.0) + Vector2f::new(0.0, size.y));

		self.window.draw_rectangle_shape(&shape, RenderStates::default());
	}

	// #[allow(unused)]
	// pub fn draw_sprite(&self, position: Vec2f, radius: Vec2f, color: Color, texture_id: Option<TextureId>) {
	// 	let texture = texture_id.map(|texture_id| self.texture_state.get_texture(texture_id));
	// 	self.draw_texture(position, radius, color, texture);
	// }

	pub fn draw_animation<T: IntoCanvasVec>(&self, position: T, radius: T, animation: Animation) {
		let texture = self.animation_state.get_animation_texture(animation);
		self.draw_texture(position, radius, Color::WHITE, Some(texture));
	}

	pub fn draw_text<T: IntoCanvasVec>(&self, position: T, size: u32, text: &str) {
		let position = position.to_canvas(self.tilemap_size);

		let font = self.font_state.get_font(FontId::DefaultFont);
		let mut text = Text::new(text, &font, size);
		text.set_position(position * (self.window.size().y as f32));
		self.window.draw_text(&text, RenderStates::default());
	}

	pub fn draw_circle(&self, position: GameVec, radius: i32, color: Color) {
		let radius = radius as f32 / TILESIZE as f32;
		let mut shape = CircleShape::new(radius, 32);
		shape.set_position(position.to_f() / TILESIZE as f32);
		shape.set_origin(Vec2f::new(radius, radius));
		shape.set_fill_color(color);

		let size = Vector2f::new(self.window.size().x as f32, self.window.size().y as f32);
		let height = self.tilemap_size.y as f32;
		let tile = size.y / height;
		shape.set_radius(shape.radius() * tile);
		shape.set_origin(shape.origin() * tile);
		shape.set_position(shape.position() * Vector2f::new(tile, -tile) + Vector2f::new(0.0, size.y));

		self.window.draw_circle_shape(&shape, RenderStates::default());
	}
}
