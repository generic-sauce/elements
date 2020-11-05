use crate::prelude::*;

const BUTTON_TEXT_SIZE: f32 = 0.05;
const EDIT_FIELD_BORDER_WIDTH: f32 = 0.004;

pub trait OnEventImpl<B: Backend>: Fn(&mut App<B>, &mut Runnable<B>) {
	fn clone_box(&self) -> Box<dyn OnEventImpl<B>>;
}

pub type OnEvent<B: Backend> = Box<dyn OnEventImpl<B>>;

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
		cursor: u32,
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

	pub fn draw(&self, draw: &mut Draw, cursor_pos: CanvasVec) {
		let color = if self.clicked {
			Color::rgb(0.18, 0.43, 0.54)
		} else if self.is_colliding(cursor_pos) {
			Color::rgb(0.12, 0.32, 0.47)
		} else {
			Color::rgb(0.08, 0.26, 0.42)
		};
		match &self.kind {
			MenuKind::Button { text, .. } => { self.draw_button(draw, text, color) },
			MenuKind::EditField { text, selected, cursor } => { self.draw_edit_field(draw, text, color, *selected, *cursor) },
		}
	}

	fn draw_button(&self, draw: &mut Draw, text: &str, color: Color) {
		draw.rectangle(self.position - self.size, self.position + self.size, color);
        draw.text(self.position - self.size, BUTTON_TEXT_SIZE, Color::WHITE, text);
	}

	fn draw_edit_field(&self, draw: &mut Draw, text: &str, color: Color, selected: bool, cursor: u32) {
        draw.rectangle(self.position - self.size, self.position + self.size, color);
        draw.rectangle(
			self.position - self.size + CanvasVec::new(EDIT_FIELD_BORDER_WIDTH, EDIT_FIELD_BORDER_WIDTH),
			self.position + self.size - CanvasVec::new(EDIT_FIELD_BORDER_WIDTH, EDIT_FIELD_BORDER_WIDTH),
			Color::rgb(0.0, 0.03, 0.15),
		);

		draw.text(self.position - self.size, BUTTON_TEXT_SIZE, Color::WHITE, text);
		/*
		if selected {
			let text_width = context.get_text_width(BUTTON_TEXT_SIZE, &text[..cursor as usize]);
		}
		context.draw_text(
			target,
			self.position - CanvasVec::new(self.size.x - 0.01, 0.45 * BUTTON_TEXT_SIZE),
			BUTTON_TEXT_SIZE,
			&text,
			Origin::LeftBottom
		);
		 */
	}

	pub fn apply_text(&mut self, event_text: &[Character]) {
		if let MenuKind::EditField { text, cursor, .. } = &mut self.kind {
			for character in event_text {
				match character {
					Character::Char(c) => {
						text.insert(*cursor as usize, *c);
						*cursor += 1;
					},
					Character::Backspace => {
						if *cursor != 0 {
							text.drain((*cursor - 1) as usize..(*cursor) as usize);
							*cursor = (*cursor - 1).max(0);
						}
					},
					Character::Delete => {
						if *cursor < text.len() as u32 {
							text.drain((*cursor) as usize..(*cursor + 1) as usize);
						}
					},
					_ => {},
				}
			}
		}
	}

	pub fn apply_key_events(&mut self, peripherals_state: &PeripheralsState) {
        if let MenuKind::EditField { cursor, .. } = &mut self.kind {
			if peripherals_state.key_just_pressed(&Key::Left) {
				*cursor = cursor.checked_sub(1).unwrap_or(0);
			}
			if peripherals_state.key_just_pressed(&Key::Right) {
				*cursor = (*cursor + 1).min(text.len() as u32);
			}
		}

	}
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
