use crate::prelude::*;

const BUTTON_TEXT_SIZE: f32 = 0.05;
const EDIT_FIELD_BORDER_WIDTH: f32 = 0.004;

pub struct MenuElement {
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
	}
}

impl MenuElement {
	pub fn new_button(position: CanvasVec, size: CanvasVec, text: &'static str, runnable_change: RunnableChange) -> MenuElement {
		MenuElement {
			kind: MenuKind::Button { runnable_change, text },
			position,
			size,
			hovered: false,
			clicked: false,
		}
	}

	pub fn new_edit_field(position: CanvasVec, size: CanvasVec, text: &str) -> MenuElement {
		MenuElement {
			kind: MenuKind::EditField { text: String::from(text), selected: false },
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
			MenuKind::EditField { text, selected } => { self.draw_edit_field(target, context, text, color, *selected) },
		}
	}

	fn draw_button(&self, target: &RenderWindow, context: &mut DrawContext, text: &str, color: Color) {
		context.draw_rect(target, self.position, self.size, color, Origin::Center);
		context.draw_text(target, self.position - CanvasVec::new(text.len() as f32 * BUTTON_TEXT_SIZE / 5.5, 0.45 * BUTTON_TEXT_SIZE), BUTTON_TEXT_SIZE, &text, Origin::LeftBottom);
	}

	fn draw_edit_field(&self, target: &RenderWindow, context: &mut DrawContext, text: &str, color: Color, selected: bool) {
		context.draw_rect(target, self.position, self.size, color, Origin::Center);
		context.draw_rect(
			target,
			self.position,
			self.size - CanvasVec::new(EDIT_FIELD_BORDER_WIDTH, EDIT_FIELD_BORDER_WIDTH),
			Color::BLACK,
			Origin::Center
		);
		if selected {
			let text_width = context.get_text_width(BUTTON_TEXT_SIZE, text);
			context.draw_rect(
				target,
				CanvasVec::new(self.position.x - self.size.x + 0.015 + text_width, self.position.y),
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

	pub fn apply_key_press(&mut self, event: &KeyPressedEvent) {
		if let MenuKind::EditField { text, .. } = &mut self.kind {
			if let Some(c) = event.to_char() {
				text.push(c);
			} else {
				if event.code == Key::BackSpace {
					text.pop();
				}
			}
		}
	}
}
