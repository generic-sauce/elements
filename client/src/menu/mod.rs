mod menu_elements;
pub use menu_elements::*;

mod events;
pub use events::*;

use crate::prelude::*;

pub const ASPECT_RATIO: f32 = 16.0 / 9.0;
const MENU_BUTTONS_WIDTH: f32 = 0.1;
const MENU_BUTTONS_HEIGHT: f32 = 0.05;

pub struct Menu<B: Backend> {
	pub elements: Vec<MenuElement<B>>,
}

fn create_local<B: Backend>(best_of_n: u32) -> OnEvent<B> {
	Box::new(move |app, runnable| {
		app.menu.elements.clear();
		*runnable = Runnable::Local(Local::new(best_of_n));
	})
}

fn create_client<B: Backend>() -> OnEvent<B> {
	Box::new(|app, runnable| {
		if let MenuKind::EditField( EditField { text, .. } ) = &app.menu.get_element_by_name("ip").unwrap().kind {
			let text = text.clone();
			app.menu.elements.clear();
			*runnable = Runnable::Client(Client::new(&text, DEFAULT_GAME_SERVER_PORT));
		} else {
			panic!("Could not read ip from edit field!");
		}
	})
}


impl<B: Backend> Menu<B> {
	pub fn new() -> Menu<B> {
		Menu {
			elements: Vec::new(),
		}
	}

	pub fn main_menu_items(selected: u8) -> Vec<MenuElement<B>> {
		let mut elements = vec![
			MenuElement::new_button(
				CanvasVec::new(MENU_BUTTONS_WIDTH, 1.0 - MENU_BUTTONS_HEIGHT),
				CanvasVec::new(MENU_BUTTONS_WIDTH, MENU_BUTTONS_HEIGHT),
				"Online",
				Color::hex("153962"),
				Box::new(create_online_menu),
			),
			MenuElement::new_button(
				CanvasVec::new(MENU_BUTTONS_WIDTH, 1.0 - (MENU_BUTTONS_HEIGHT * 3.0)),
				CanvasVec::new(MENU_BUTTONS_WIDTH, MENU_BUTTONS_HEIGHT),
				"Local",
				Color::hex("153962"),
				Box::new(create_local_menu)
			),
			MenuElement::new_button(
				CanvasVec::new(MENU_BUTTONS_WIDTH, 1.0 - (MENU_BUTTONS_HEIGHT * 5.0)),
				CanvasVec::new(MENU_BUTTONS_WIDTH, MENU_BUTTONS_HEIGHT),
				"Tutorial",
				Color::hex("153962"),
				Box::new(create_tutorial_menu)
			),
			MenuElement::new_button(
				CanvasVec::new(MENU_BUTTONS_WIDTH, MENU_BUTTONS_HEIGHT),
				CanvasVec::new(MENU_BUTTONS_WIDTH, MENU_BUTTONS_HEIGHT),
				"Quit",
				Color::hex("0c2542"),
				Box::new(|_, _| std::process::exit(0))
			),
		];
		elements[selected as usize].color = Color::hex("295e9a");
		elements
	}

	pub fn online_menu() -> Menu<B> {
		let mut elements = Menu::main_menu_items(0);
		elements.extend(vec![
			MenuElement::new_button(
				CanvasVec::new(0.5 * ASPECT_RATIO, 0.4),
				CanvasVec::new(0.15, 0.05),
				"Play Now",
				Color::hex("116201"),
				Box::new(create_server_connector)
			),
			MenuElement::new_edit_field(
				"player_name",
				CanvasVec::new(0.5 * ASPECT_RATIO, 0.6),
				CanvasVec::new(0.15, 0.03),
				"",
				DEFAULT_BUTTON_COLOR
			)
		]);
		Menu {
			elements,
		}
	}

	pub fn local_menu() -> Menu<B> {
		let mut elements = Menu::main_menu_items(1);
		elements.extend(vec![
			MenuElement::new_button(
				CanvasVec::new(0.5 * ASPECT_RATIO, 0.4),
				CanvasVec::new(0.15, 0.05),
				"Start Game",
				Color::hex("116201"),
				Box::new(create_local(5)),
			),
		]);
		Menu {
			elements
		}
	}

	pub fn tutorial_menu() -> Menu<B> {
		let elements = Menu::main_menu_items(2);
		// TODO: add tutorial menu
		Menu {
			elements,
		}
	}

	pub fn server_connector_menu() -> Menu<B> {
		Menu {
			elements: vec!(
				MenuElement::new_button(CanvasVec::new(0.5 * ASPECT_RATIO, 0.4), CanvasVec::new(0.15, 0.05), "Connecting", DEFAULT_BUTTON_COLOR, Box::new(noop)),
			)
		}
	}

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
				MenuKind::Button { on_click, .. } => {
					opt_on_click = Some(on_click.clone());
				}
				MenuKind::EditField( EditField { selected, .. } ) => {
					*selected = true;
				}
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

	pub fn draw_menu(&mut self) {
		let mut draw = Draw::new();
		// draw.set_clear_color(Color::BLACK);
		draw.texture(ViewVec::new(0.0, 0.0), ViewVec::new(1.0, 1.0), TextureId::SkyBackground, Flip::Normal, Some(Color::rgb(0.6, 0.6, 0.6)));

		// draw elements
		for element in &mut self.menu.elements {
			element.draw(&mut draw, self.cursor_position, &self.graphics_backend);
		}

		// draw cursor
		draw.rectangle(self.cursor_position, self.cursor_position + CanvasVec::new(0.01, 0.01), Color::RED);

		self.graphics_backend.submit(draw);
	}
}
