use crate::prelude::*;
use std::ops::{Add, Sub, Mul};

const BUTTON_TEXT_SIZE: f32 = 0.08;
const EDIT_FIELD_BORDER_WIDTH: f32 = 0.004;

pub trait OnEventImpl<B: Backend>: Fn(&mut App<B>, &mut Runnable<B>) {
	fn clone_box(&self) -> Box<dyn OnEventImpl<B>>;
}

pub type OnEvent<B> = Box<dyn OnEventImpl<B>>;

pub struct MenuElement<B: Backend> {
	pub name: &'static str,
	pub kind: MenuKind<B>,
	pub position: CanvasVec,
	pub size: CanvasVec,
	pub hovered: bool,
	pub clicked: bool,
}

pub enum MenuKind<B: Backend> {
	Button {
		text: &'static str,
		on_click: OnEvent<B>,
	},
	EditField {
		text: String,
		selected: bool,
		cursor: usize,
	}
}

impl<B: Backend> MenuElement<B> {
	pub fn new_button(position: CanvasVec, size: CanvasVec, text: &'static str, on_click: OnEvent<B>) -> MenuElement<B> {
		MenuElement {
			name: "",
			kind: MenuKind::Button { text, on_click },
			position,
			size,
			hovered: false,
			clicked: false,
		}
	}

	pub fn new_edit_field(name: &'static str, position: CanvasVec, size: CanvasVec, text: &str) -> MenuElement<B> {
		MenuElement {
			name,
			kind: MenuKind::EditField { text: String::from(text), selected: false, cursor: 0 },
			position,
			size,
			hovered: false,
			clicked: false,
		}
	}

	pub fn is_colliding(&self, pos: CanvasVec) -> bool {
		pos.x >= self.position.x - self.size.x && pos.x <= self.position.x + self.size.x &&
		pos.y >= self.position.y - self.size.y && pos.y <= self.position.y + self.size.y
	}

	pub fn draw(&self, draw: &mut Draw, cursor_pos: CanvasVec, graphics_backend: &impl GraphicsBackend) {
		let color = if self.clicked {
			Color::rgb(0.18, 0.43, 0.54)
		} else if self.is_colliding(cursor_pos) {
			Color::rgb(0.12, 0.32, 0.47)
		} else {
			Color::rgb(0.08, 0.26, 0.42)
		};
		match &self.kind {
			MenuKind::Button { text, .. } => {
				self.draw_button(draw, text, color, graphics_backend)
			},
			MenuKind::EditField { text, selected, cursor } => {
				self.draw_edit_field(draw, text, color, *selected, *cursor, graphics_backend)
			},
		}
	}

	fn draw_button(&self, draw: &mut Draw, text: &str, color: Color, graphics_backend: &impl GraphicsBackend) {
		let left_bot = self.position - self.size;
		let right_top = self.position + self.size;
		let text_pos = center_position(left_bot, right_top, graphics_backend.get_text_width(text) * BUTTON_TEXT_SIZE);
		draw.rectangle(left_bot, right_top, color);
        draw.text(text_pos, BUTTON_TEXT_SIZE, Color::WHITE, text);
	}

	fn draw_edit_field(&self, draw: &mut Draw, text: &str, color: Color, selected: bool, cursor: usize, graphics_backend: &impl GraphicsBackend) {
        draw.rectangle(self.position - self.size, self.position + self.size, color);
        draw.rectangle(
			self.position - self.size + CanvasVec::new(EDIT_FIELD_BORDER_WIDTH, EDIT_FIELD_BORDER_WIDTH),
			self.position + self.size - CanvasVec::new(EDIT_FIELD_BORDER_WIDTH, EDIT_FIELD_BORDER_WIDTH),
			Color::rgb(0.0, 0.03, 0.15),
		);

		let text_pos = self.position - self.size + CanvasVec::new(0.008, -0.012);

		draw.text(text_pos, BUTTON_TEXT_SIZE, Color::WHITE, text);

		if selected {
			let subtext = &text[0..get_byte_pos(text, cursor)];
			let text_width = graphics_backend.get_text_width(subtext).x;
			let left_bot = text_pos + CanvasVec::new(text_width * BUTTON_TEXT_SIZE + -0.001, 0.017);
			draw.rectangle(
				left_bot,
				left_bot + CanvasVec::new(0.002, BUTTON_TEXT_SIZE*0.6),
				Color::WHITE,
			);
		}
	}

	pub fn apply_text(&mut self, event_text: &[Character]) {
		if let MenuKind::EditField { text, cursor, .. } = &mut self.kind {
			for character in event_text {
				match character {
					Character::Char(c) => {
						text.insert(get_byte_pos(text, *cursor), *c);
						*cursor += 1;
					},
					Character::Backspace => {
						if *cursor != 0 {
							text.drain(get_byte_pos(text, *cursor - 1)..get_byte_pos(text, *cursor));
							*cursor = (*cursor - 1).max(0);
						}
					},
					Character::Delete => {
						if *cursor < text.chars().count() {
							text.drain(get_byte_pos(text, *cursor)..get_byte_pos(text, *cursor + 1));
						}
					},
					_ => {},
				}
			}
		}
	}

	pub fn apply_key_events(&mut self, peripherals_state: &PeripheralsState) {
        if let MenuKind::EditField { cursor, text, .. } = &mut self.kind {
			if peripherals_state.key_firing(Key::Left) {
				*cursor = cursor.checked_sub(1).unwrap_or(0);
			}
			if peripherals_state.key_firing(Key::Right) {
				*cursor = (*cursor + 1).min(text.len());
			}
		}

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
