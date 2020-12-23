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
				}
				MenuKind::EditField( EditField { selected, .. } ) => {
					*selected = true;
				}
				MenuKind::Label(_) => {}
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

	pub fn draw_menu(&mut self, draw: &mut Draw) {
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
