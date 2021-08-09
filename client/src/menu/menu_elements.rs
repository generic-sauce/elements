use crate::prelude::*;
use std::ops::{Add, Sub, Mul};

pub const MAIN_BUTTON_FONT_SIZE: f32 = 0.03;
pub const NORMAL_BUTTON_FONT_SIZE: f32 = 0.04;
pub const GO_BUTTON_FONT_SIZE: f32 = 0.05;
const EDIT_FIELD_TEXT_SIZE: f32 = 0.03;
const EDIT_FIELD_BORDER_WIDTH: f32 = 0.004;
const EDIT_FIELD_CURSOR_WIDTH: f32 = 0.002;
const EDIT_FIELD_CURSOR_BLINK_INTERVAL: u32 = 60;
pub const EXPLANATION_FONT_SIZE: f32 = 0.03;
pub const SUBTITLE_FONT_SIZE: f32 = 0.05;
pub const DEFAULT_BUTTON_COLOR: Color = Color::rgb(0.08, 0.26, 0.42);
pub const BUTTON_IMAGE_SIZE: CanvasVec = CanvasVec::new(0.04, 0.04);

pub struct MenuElement<B: Backend> {
	pub name: &'static str,
	pub kind: MenuKind<B>,
	pub position: CanvasVec,
	pub size: CanvasVec,
	pub hovered: bool,
	pub clicked: bool,
	pub color: Color,
}

pub struct Button<B: Backend> {
	pub text: &'static str,
	pub on_click: OnEvent<B>,
	pub font_size: f32,
	pub image: Option<TextureId>,
}

pub struct EditField {
	pub text: String,
	pub template_text: String,
	pub selected: bool,
	pub cursor: usize,
	pub cursor_blink_counter: u32,
	pub view_offset: usize,
	pub view_limit: usize,
	pub font_size: f32,
}

pub enum TextAlign {
	Left,
	Center,
}

pub struct Label {
	pub text: String,
	pub align: TextAlign,
	pub font_size: f32,
}

pub struct Image {
	pub texture_index: TextureIndex
}

pub enum MenuKind<B: Backend> {
	Button(Button<B>),
	EditField(EditField),
	Label(Label),
	Image(Image),
}

impl<B: Backend> MenuElement<B> {
	pub fn new_button(position: CanvasVec, size: CanvasVec, text: &'static str, color: Color, font_size: f32, image: Option<TextureId>, on_click: OnEvent<B>) -> MenuElement<B> {
		MenuElement {
			name: "",
			kind: MenuKind::Button(Button { text, on_click, font_size, image } ),
			position,
			size,
			hovered: false,
			clicked: false,
			color,
		}
	}

	pub fn new_edit_field(name: &'static str, position: CanvasVec, size: CanvasVec, text: String, color: Color, template_text: &str) -> MenuElement<B> {
		MenuElement {
			name,
			kind: MenuKind::EditField( EditField::new(text, template_text) ),
			position,
			size,
			hovered: false,
			clicked: false,
			color,
		}
	}

	pub fn new_label(position: CanvasVec, size: CanvasVec, font_size: f32, text: &str, align: TextAlign) -> MenuElement<B> {
		MenuElement {
			name: "",
			kind: MenuKind::Label(Label {
				text: text.to_string(),
				align,
				font_size,
			}),
			position,
			size,
			hovered: false,
			clicked: false,
			color: Color::WHITE
		}
	}

	pub fn new_image(position: CanvasVec, size: CanvasVec, texture_index: impl IntoTextureIndex) -> MenuElement<B> {
		MenuElement {
			name: "",
			kind: MenuKind::Image(Image {
				texture_index: texture_index.into_texture_index(),
			}),
			position,
			size,
			hovered: false,
			clicked: false,
			color: Color::WHITE
		}
	}

	pub fn is_colliding(&self, pos: CanvasVec) -> bool {
		pos.x >= self.position.x - self.size.x && pos.x <= self.position.x + self.size.x &&
		pos.y >= self.position.y - self.size.y && pos.y <= self.position.y + self.size.y
	}

	pub fn tick(&mut self, graphics_backend: &impl GraphicsBackend) {
		if let MenuKind::EditField (edit_field) = &mut self.kind {
			edit_field.cursor_blink_counter = (edit_field.cursor_blink_counter + 1) % EDIT_FIELD_CURSOR_BLINK_INTERVAL;

			// view offset
			edit_field.adapt_view(graphics_backend, self.size);
		}
	}

	pub fn draw(&mut self, draw: &mut Draw, cursor_pos: CanvasVec, graphics_backend: &impl GraphicsBackend) {
		let color = if self.clicked {
			self.color * 2.0
		} else if self.is_colliding(cursor_pos) {
			self.color * 1.5
		} else {
			self.color
		};
		match &self.kind {
			MenuKind::Button(button) => {
				self.draw_button(draw, button, color, graphics_backend)
			},
			MenuKind::EditField (edit_field) => {
				self.draw_edit_field(draw, edit_field, color, graphics_backend)
			},
			MenuKind::Label(label) => {
				self.draw_label(draw, label, color, graphics_backend)
			},
			MenuKind::Image(image) => {
				self.draw_image(draw, image, color)
			},
		}
	}

	fn draw_button(&self, draw: &mut Draw, button: &Button<B>, color: Color, graphics_backend: &impl GraphicsBackend) {
		let Button { text, font_size, image, .. } = button;
		let left_bot = self.position - self.size;
		let right_top = self.position + self.size;
		draw.rectangle(left_bot, right_top, color);

		let mut text_pos = center_position(left_bot, right_top, graphics_backend.get_text_size(text, *font_size));
		if image.is_some() {
			text_pos.y -= 0.03;
		}
		draw.text(text_pos, *font_size, Color::WHITE, text);

		if let Some(texture_id) = image {
			let mut image_pos = center_position(left_bot, right_top, BUTTON_IMAGE_SIZE);
			image_pos.y += 0.014;
			draw.texture(image_pos, image_pos + BUTTON_IMAGE_SIZE, *texture_id, Flip::Normal, None);
		}
	}

	fn draw_edit_field(&self, draw: &mut Draw, edit_field: &EditField, color: Color, graphics_backend: &impl GraphicsBackend) {
		let EditField { cursor_blink_counter, cursor, selected, view_offset, template_text, font_size, .. } = edit_field;
		draw.rectangle(self.position - self.size, self.position + self.size, color);
		draw.rectangle(
			self.position - self.size + EDIT_FIELD_BORDER_WIDTH,
			self.position + self.size - EDIT_FIELD_BORDER_WIDTH,
			Color::rgb(0.0, 0.03, 0.15),
		);

		let text = edit_field.get_render_text();

		let text_size = if !text.is_empty() {
			graphics_backend.get_text_size(text, *font_size)
		} else {
			graphics_backend.get_text_size(&template_text, *font_size)
		};

		let text_pos = CanvasVec::new(
			self.position.x - self.size.x + EDIT_FIELD_BORDER_WIDTH * 2.7,
			center_position(self.position.y - self.size.y, self.position.y + self.size.y, text_size.y)
		);

		if edit_field.text.is_empty() {
			draw.text(text_pos, *font_size, Color::gray(0.04), &template_text);
		} else {
			draw.text(text_pos, *font_size, Color::WHITE, text);
		}

		// draw cursor
		if *selected && *cursor_blink_counter < EDIT_FIELD_CURSOR_BLINK_INTERVAL / 2 {
			let subtext = &text[0..get_byte_pos(text, *cursor - view_offset)];
			let text_width = graphics_backend.get_text_size(subtext, *font_size).x;
			let left_bot = CanvasVec::new(
				self.position.x - self.size.x + text_width + EDIT_FIELD_BORDER_WIDTH * 2.0,
				self.position.y - self.size.y + EDIT_FIELD_BORDER_WIDTH * 2.0
			);
			let right_top = CanvasVec::new(
				left_bot.x + EDIT_FIELD_CURSOR_WIDTH,
				self.position.y + self.size.y - EDIT_FIELD_BORDER_WIDTH * 2.0
			);
			draw.rectangle(left_bot, right_top, Color::WHITE);
		}
	}

	fn draw_label(&self, draw: &mut Draw, label: &Label, color: Color, graphics_backend: &impl GraphicsBackend) {
		let Label { text, font_size, align, .. } = label;

		let text_size = graphics_backend.get_text_size(text, *font_size);
		let text_pos = match align {
			TextAlign::Left => CanvasVec::new(
				self.position.x - self.size.x,
				center_position(self.position.y - self.size.y, self.position.y + self.size.y, text_size.y)
			),
			TextAlign::Center => center_position::<CanvasVec>(self.position - self.size, self.position + self.size, text_size),
		};

		draw.text(text_pos, *font_size, color, text);
	}

	fn draw_image(&self, draw: &mut Draw, image: &Image, color: Color) {
		let Image { texture_index } = *image;
		let radius = self.size / 2.0;
		let left_bot = self.position - radius;
		let right_top = self.position + radius;
		draw.texture(left_bot, right_top, texture_index, Flip::Normal, Some(color));
	}

	pub fn apply_text(&mut self, event_text: &[Character]) {
		if let MenuKind::EditField ( EditField { text, cursor, cursor_blink_counter, .. } ) = &mut self.kind {
			for character in event_text {
				match character {
					Character::Char(c) => {
						text.insert(get_byte_pos(text, *cursor), *c);
						*cursor += 1;
						*cursor_blink_counter = 0;
					},
					Character::Backspace => {
						if *cursor != 0 {
							text.drain(get_byte_pos(text, *cursor - 1)..get_byte_pos(text, *cursor));
							*cursor = (*cursor - 1).max(0);
						}
						*cursor_blink_counter = 0;
					},
					Character::Delete => {
						if *cursor < text.chars().count() {
							text.drain(get_byte_pos(text, *cursor)..get_byte_pos(text, *cursor + 1));
						}
						*cursor_blink_counter = 0;
					},
					_ => {},
				}
			}
		}
	}

	pub fn apply_key_events(&mut self, peripherals_state: &PeripheralsState) {
		if let MenuKind::EditField( EditField{ cursor, text, cursor_blink_counter, .. } ) = &mut self.kind {
			if peripherals_state.key_firing(Key::Left) {
				*cursor = cursor.checked_sub(1).unwrap_or(0);
				*cursor_blink_counter = 0;
			}
			if peripherals_state.key_firing(Key::Right) {
				*cursor = (*cursor + 1).min(text.len());
				*cursor_blink_counter = 0;
			}
		}
	}
}

impl EditField {
	fn new(text: String, template_text: &str) -> EditField {
		EditField {
			text,
			template_text: String::from(template_text),
			selected: false,
			cursor: 0,
			cursor_blink_counter: 0,
			view_offset: 0,
			view_limit: 0,
			font_size: EDIT_FIELD_TEXT_SIZE,
		}
	}

	fn adapt_view(&mut self, graphics_backend: &impl GraphicsBackend, size: CanvasVec) {
		let allowed_width = size.x * 2.0 - EDIT_FIELD_BORDER_WIDTH * 2.0;

		// if text is not right aligned -> decrease view offset
		while graphics_backend.get_text_size(self.get_text_post_view_offset(), self.font_size).x <= allowed_width - 0.03 {
			if self.view_offset == 0 {
				break;
			}
			self.view_offset -= 1;
		}

		// test if cursor is left out of view range
		// -> decrease view offset
		if self.cursor < self.view_offset {
			self.view_offset = self.cursor;
		} else {
			// test if cursor is right out of view range
			// -> increase view offset
			while self.get_cursor_render_offset(graphics_backend) > allowed_width {
				self.view_offset += 1;
			}
		}


		// view limit
		self.view_limit = self.view_limit.min(self.text.len());
		while graphics_backend.get_text_size(self.get_render_text(), self.font_size).x <= allowed_width {
			if self.view_limit == self.text.len() {
				break;
			}
			self.view_limit += 1;
		}
		while graphics_backend.get_text_size(self.get_render_text(), self.font_size).x > allowed_width {
			if self.view_limit == 0 {
				break;
			}
			self.view_limit -= 1;
		}
	}

	fn get_render_text(&self) -> &str {
		&self.text[get_byte_pos(&self.text, self.view_offset)..get_byte_pos(&self.text, self.view_limit)]
	}

	fn get_cursor_render_offset(&self, graphics_backend: &impl GraphicsBackend) -> f32 {
		graphics_backend.get_text_size(self.get_pre_cursor_text(), self.font_size).x
	}

	// text after view offset but before cursor
	fn get_pre_cursor_text(&self) -> &str {
		&self.text[get_byte_pos(&self.text, self.view_offset)..get_byte_pos(&self.text, self.cursor)]
	}

	fn get_text_post_view_offset(&self) -> &str {
		&self.text[get_byte_pos(&self.text, self.view_offset)..]
	}
}

fn center_position<T>(outer_left: T, outer_right: T, inner_size: T) -> T
	where T: Add<Output=T> + Sub<Output=T> + Mul<f32, Output=T> + Copy
{
	let outer_size = outer_right - outer_left;
	let space = (outer_size - inner_size) * 0.5;
	outer_left + space
}

fn get_byte_pos(text: &str, char_pos: usize) -> usize {
	text.char_indices()
		.nth(char_pos)
		.map(|(i, _)| i)
		.unwrap_or(text.len())
}

// OnEvent impl
impl<B: Backend, F: Fn(&mut App<B>, &mut Runnable<B>) + Clone + 'static> OnEventImpl<B> for F {
	fn clone_box(&self) -> Box<dyn OnEventImpl<B>> {
		Box::new(self.clone())
	}
}

impl<B: Backend> Clone for Box<dyn OnEventImpl<B>> {
	fn clone(&self) -> Self { (**self).clone_box() }
}
