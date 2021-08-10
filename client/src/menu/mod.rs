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

pub struct MenuCache {
	pub clicked_element: Option<String>, // by name, the mouse is currently holding the click
	pub selected_element: Option<String>, // by name, edit field that is selected
	pub hovered_element: Option<String>, // by name
	pub edit_field: HashMap<String, EditFieldCache>,
}

impl MenuCache {
	pub fn new() -> MenuCache {
		MenuCache {
			clicked_element: None,
			selected_element: None,
			hovered_element: None,
			edit_field: HashMap::new(),
		}
	}
}

pub struct EditFieldCache {
	pub text: String,
	pub cursor: usize,
	pub cursor_blink_counter: u32,
	pub view_offset: usize,
	pub view_limit: usize,
}

impl<B: Backend> Menu<B> {
	pub fn get_clicked_element(&mut self, menu_cache: &MenuCache) -> Option<&mut MenuElement<B>> {
		menu_cache.clicked_element.as_ref().and_then(move |x| self.get_element_by_name(x))
	}

	pub fn get_selected_element(&mut self, menu_cache: &MenuCache) -> Option<&mut MenuElement<B>> {
		menu_cache.selected_element.as_ref().and_then(move |x| self.get_element_by_name(&x))
	}

	pub fn get_element_by_name(&mut self, name: &str) -> Option<&mut MenuElement<B>> {
		self.elements.iter_mut().find(|e| e.name == name)
	}

	pub fn calc_element_properties(&mut self, app: &mut App<B>, runnable: &mut Runnable<B>) -> Option<OnEvent<B>> { // updates hover & is_clicked properties of menues
		app.check_menu_active(runnable);
		if !runnable.is_active() { return None; } // dont tick, if inactive

		let hovered_element = self.elements.iter().find(|e| e.is_colliding(app.cursor_position));
		let opt_name = hovered_element.map(|x| x.name.clone());
		app.menu_cache.hovered_element = opt_name.clone();

		let mut opt_on_click = None;

		if app.peripherals_state.key_just_pressed(Key::LeftMouse) {
			app.menu_cache.selected_element = opt_name.clone();
			app.menu_cache.clicked_element = opt_name.clone();
		} else if !app.peripherals_state.key_pressed(Key::LeftMouse) {
			if let Some(clicked) = app.menu_cache.clicked_element.take() {
				let clicked = self.get_element_by_name(&clicked);
				if let Some(MenuElement { kind: MenuKind::Button( Button { on_click, .. } ), .. } ) = clicked {
					opt_on_click = Some(on_click.clone());
				}
			}
		}

		if let Some(element) = self.get_selected_element(&app.menu_cache) {
			let edit_field_cache = app.menu_cache.edit_field.get_mut(&element.name).unwrap(); // TODO: this will crash, if other elements can be selected
			element.apply_text(&app.peripherals_state.text, edit_field_cache);
			element.apply_key_events(&app.peripherals_state, edit_field_cache);
		}

		for element in &mut self.elements {
			element.tick(&app.graphics_backend, &mut app.menu_cache);
		}

		opt_on_click
	}

	pub fn init_cache(&self, menu_cache: &mut MenuCache) {
		for element in &self.elements {
			match element.kind {
				MenuKind::EditField(_) => {
					if element.name.is_empty() {
						panic!("ERROR: edit field with empty name!");
					}
					if !menu_cache.edit_field.contains_key(&element.name) {
						menu_cache.edit_field.insert(element.name.clone(), EditFieldCache {
							text: "".to_string(),
							cursor: 0,
							cursor_blink_counter: 0,
							view_offset: 0,
							view_limit: 0,
						});
					}
				},
				_ => {}
			}
		}
	}
}

impl<B: Backend> App<B> {
	fn check_menu_active(&self, runnable: &mut Runnable<B>) {
		if self.peripherals_state.key_just_pressed(Key::Escape) || self.peripherals_state.key_just_pressed(Key::P) {
			runnable.toggle_active();
		}
	}

	pub fn draw_menu(&mut self, menu: &Menu<B>, draw: &mut Draw, runnable: &Runnable<B>) {
		if !runnable.is_active() { return; }

		if let Some(texture_id) = menu.background {
			draw.texture(ViewVec::new(0.0, 0.0), ViewVec::new(1.0, 1.0), texture_id, Flip::Normal, Some(Color::rgb(0.8, 0.8, 0.8)));
		}

		// draw elements
		for element in &menu.elements {
			element.draw(draw, self.cursor_position, &self.graphics_backend, &self.menu_cache);
		}

		// draw cursor
		draw.circle(self.cursor_position, CURSOR_RADIUS, Color::RED);
	}
}