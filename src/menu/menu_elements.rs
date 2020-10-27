use crate::prelude::*;

const BUTTON_TEXT_SIZE: f32 = 0.05;
const EDIT_FIELD_BORDER_WIDTH: f32 = 0.004;

pub struct MenuElement {
	pub name: &'static str,
	pub kind: MenuKind,
	pub position: CanvasVec,
	pub size: CanvasVec,
	pub hovered: bool,
	pub clicked: bool,
}

pub enum MenuKind {
	Button {
		text: &'static str,
		runnable_change: RunnableChange,
	},
	EditField {
		text: String,
		selected: bool,
		cursor: u32,
	}
}

impl MenuElement {
	pub fn new_button(position: CanvasVec, size: CanvasVec, text: &'static str, runnable_change: RunnableChange) -> MenuElement {
		MenuElement {
			name: "",
			kind: MenuKind::Button { runnable_change, text },
			position,
			size,
			hovered: false,
			clicked: false,
		}
	}

	pub fn new_edit_field(name: &'static str, position: CanvasVec, size: CanvasVec, text: &str) -> MenuElement {
		MenuElement {
			name,
			kind: MenuKind::EditField { text: String::from(text), selected: false, cursor: 0 },
			position,
			size,
			hovered: false,
			clicked: false,
		}
	}

	pub fn is_colliding(&self, pos: &CanvasVec) -> bool {
		pos.x >= self.position.x - self.size.x && pos.x <= self.position.x + self.size.x &&
		pos.y >= self.position.y - self.size.y && pos.y <= self.position.y + self.size.y
	}

	pub fn draw(&self, target: &RenderWindow, context: &mut DrawContext, cursor_pos: &CanvasVec) {
		let color = if self.clicked {
			Color::rgb(47, 110, 140)
		} else if self.is_colliding(cursor_pos) {
			Color::rgb(32, 82, 120)
		} else {
			Color::rgb(21, 67, 109)
		};
		match &self.kind {
			MenuKind::Button { text, runnable_change: _runnable_change } => { self.draw_button(target, context, text, color) },
			MenuKind::EditField { text, selected, cursor } => { self.draw_edit_field(target, context, text, color, *selected, *cursor) },
		}
	}

	fn draw_button(&self, target: &RenderWindow, context: &mut DrawContext, text: &str, color: Color) {
		context.draw_rect(target, self.position, self.size, color, Origin::Center);
		context.draw_text(target, self.position - CanvasVec::new(text.len() as f32 * BUTTON_TEXT_SIZE / 5.5, 0.45 * BUTTON_TEXT_SIZE), BUTTON_TEXT_SIZE, &text, Origin::LeftBottom);
	}

	fn draw_edit_field(&self, target: &RenderWindow, context: &mut DrawContext, text: &str, color: Color, selected: bool, cursor: u32) {
		context.draw_rect(target, self.position, self.size, color, Origin::Center);
		context.draw_rect(
			target,
			self.position,
			self.size - CanvasVec::new(EDIT_FIELD_BORDER_WIDTH, EDIT_FIELD_BORDER_WIDTH),
			Color::rgb(0, 10, 40),
			Origin::Center
		);
		if selected {
			let text_width = context.get_text_width(BUTTON_TEXT_SIZE, &text[..cursor as usize]);
			context.draw_rect(
				target,
				CanvasVec::new(self.position.x - self.size.x + 0.013 + text_width, self.position.y),
				CanvasVec::new(0.001, self.size.y * 0.7),
				Color::WHITE,
				Origin::Center
			);
		}
		context.draw_text(
			target,
			self.position - CanvasVec::new(self.size.x - 0.01, 0.45 * BUTTON_TEXT_SIZE),
			BUTTON_TEXT_SIZE,
			&text,
			Origin::LeftBottom
		);
	}

	/*
	pub fn apply_key_press(&mut self, event: &KeyPressedEvent) {
		if let MenuKind::EditField { text, cursor, .. } = &mut self.kind {
			if let Some(c) = event.to_char() {
				text.push(c);
				*cursor += 1;
			} else if event.code == Key::BackSpace {
				if *cursor != 0 {
					text.drain((*cursor - 1) as usize..(*cursor) as usize);
					*cursor = (*cursor - 1).max(0);
				}
			} else if event.code == Key::Delete {
				if *cursor < text.len() as u32 {
					text.drain((*cursor) as usize..(*cursor + 1) as usize);
				}
			} else if event.code == Key::Left {
				*cursor = cursor.checked_sub(1).unwrap_or(0);
			} else if event.code == Key::Right {
				*cursor = (*cursor + 1).min(text.len() as u32);
			}
		}
	}
	 */
}
