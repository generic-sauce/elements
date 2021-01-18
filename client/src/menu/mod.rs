mod menu_elements;
pub use menu_elements::*;

mod events;
pub use events::*;

mod menues;
pub use menues::*;

use crate::prelude::*;

pub const ASPECT_RATIO: f32 = 16.0 / 9.0;

pub struct Menu<B: Backend> {
	pub elements: Vec<MenuElement<B>>,
	pub background: Option<TextureId>,
	pub kind: MenuMode,
}

pub enum MenuMode {
	Normal,
	InGame {
		active: bool
	},
}

impl<B: Backend> Menu<B> {
	pub fn get_clicked_element(&mut self) -> Option<&mut MenuElement<B>> {
		self.elements.iter_mut().find(|e| e.clicked)
	}

	pub fn get_selected_element(&mut self) -> Option<&mut MenuElement<B>> {
		self.elements.iter_mut().find(|e| if let MenuKind::EditField(EditField { selected, .. } ) = e.kind { selected } else { false })
	}

	pub fn get_element_by_name(&mut self, name: &'static str) -> Option<&mut MenuElement<B>> {
		self.elements.iter_mut().find(|e| e.name == name)
	}
}

impl<B: Backend> App<B> {
	pub fn tick_menu(&mut self, runnable: &mut Runnable<B>) {
		self.check_menu_active();
		if !self.menu.kind.is_active() { return; } // dont tick, if inactive

		let mut opt_on_click = None;
		if self.peripherals_state.key_pressed(Key::LeftMouse) {
			for element in &mut self.menu.elements {
				element.clicked = element.is_colliding(self.cursor_position);
			}
			if let Some(elem) = self.menu.get_selected_element() {
				if let MenuKind::EditField( EditField { selected, .. } ) = &mut elem.kind {
					*selected = false;
				}
			}
		} else if let Some(element) = self.menu.get_clicked_element() {
			element.clicked = false;
			match &mut element.kind {
				MenuKind::Button(Button { on_click, ..}) => {
					opt_on_click = Some(on_click.clone());
				},
				MenuKind::EditField( EditField { selected, .. } ) => {
					*selected = true;
				},
				MenuKind::Label(_) => {},
				MenuKind::Image(_) => {},
			}
		}

		if let Some(on_click) = opt_on_click {
			on_click(self, runnable);
		}

		if let Some(element) = self.menu.get_selected_element() {
			element.apply_text(&self.peripherals_state.text);
			element.apply_key_events(&self.peripherals_state);
		}

		for element in &mut self.menu.elements {
			element.tick(&self.graphics_backend);
		}
	}

	fn check_menu_active(&mut self) {
		if let MenuMode::InGame { active } = self.menu.kind {
			// TODO: add controller key
			if self.peripherals_state.key_just_pressed(Key::Escape) || self.peripherals_state.key_just_pressed(Key::P) {
				self.menu.kind = MenuMode::InGame { active: !active };
			}
		}
	}

	pub fn draw_menu(&mut self, draw: &mut Draw) {
		if !self.menu.kind.is_active() { return; }

		if let Some(texture_id) = self.menu.background {
			#[cfg(target_arch = "wasm32")]
			draw.set_clear_color(Color::BLACK);

			#[cfg(not(target_arch = "wasm32"))]
			draw.texture(ViewVec::new(0.0, 0.0), ViewVec::new(1.0, 1.0), texture_id, Flip::Normal, Some(Color::rgb(0.8, 0.8, 0.8)));
		}

		// draw elements
		for element in &mut self.menu.elements {
			element.draw(draw, self.cursor_position, &self.graphics_backend);
		}

		// draw cursor
		draw.rectangle(self.cursor_position, self.cursor_position + CanvasVec::new(0.01, 0.01), Color::RED);
	}
}

impl MenuMode {
	pub fn is_active(&self) -> bool {
		match self {
			MenuMode::InGame { active: false } => false,
			_ => true,
		}
	}
}
