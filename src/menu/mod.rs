mod menu_elements;

pub use menu_elements::*;
use crate::prelude::*;

// pub const DEFAULT_CURSOR_POSITION: CanvasVec = CanvasVec::new(0.5 * 16.0 / 9.0, 0.5);
pub const ASPECT_RATIO: f32 = 16.0 / 9.0;

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
		if let MenuKind::EditField { text, .. } = &app.menu.get_element_by_name("ip").unwrap().kind {
			let text = text.clone();
			app.menu.elements.clear();
			*runnable = Runnable::Client(Client::new(&text));
		} else {
			panic!("Could not read ip from edit field!");
		}
	})
}

fn create_join_server<B: Backend>(app: &mut App<B>, _runnable: &mut Runnable<B>) {
	app.menu = Menu::connect_server_menu();
}

fn create_main_menu<B: Backend>(app: &mut App<B>, runnable: &mut Runnable<B>) {
	*runnable = Runnable::Menu;
	app.menu = Menu::main_menu();
}

impl<B: Backend> Menu<B> {
	pub fn new() -> Menu<B> {
		Menu {
			elements: Vec::new(),
		}
	}

	pub fn main_menu() -> Menu<B> {
		Menu {
			elements: vec!(
				MenuElement::new_button(CanvasVec::new(0.3 * ASPECT_RATIO, 0.6), CanvasVec::new(0.15, 0.05), "Best of 9", create_local(9)),
				MenuElement::new_button(CanvasVec::new(0.3 * ASPECT_RATIO, 0.4), CanvasVec::new(0.15, 0.05), "Best of 5", create_local(5)),
				MenuElement::new_button(CanvasVec::new(0.7 * ASPECT_RATIO, 0.6), CanvasVec::new(0.15, 0.05), "Infinite Game", create_local(0)),
				MenuElement::new_button(CanvasVec::new(0.7 * ASPECT_RATIO, 0.4), CanvasVec::new(0.15, 0.05), "Join Server", Box::new(create_join_server)),
				MenuElement::new_button(CanvasVec::new(0.85 * ASPECT_RATIO, 0.15), CanvasVec::new(0.15, 0.05), "Quit", Box::new(|_, _| std::process::exit(0))),
			),
		}
	}

	pub fn connect_server_menu() -> Menu<B> {
		Menu {
			elements: vec!(
				MenuElement::new_button(CanvasVec::new(0.5 * ASPECT_RATIO, 0.4), CanvasVec::new(0.15, 0.05), "Connect", create_client()),
				MenuElement::new_button(CanvasVec::new(0.15 * ASPECT_RATIO, 0.15), CanvasVec::new(0.15, 0.05), "Back", Box::new(create_main_menu)),
				MenuElement::new_button(CanvasVec::new(0.85 * ASPECT_RATIO, 0.15), CanvasVec::new(0.15, 0.05), "Quit", Box::new(|_, _| std::process::exit(0))),
				MenuElement::new_edit_field("ip", CanvasVec::new(0.5 * ASPECT_RATIO, 0.6), CanvasVec::new(0.15, 0.03), ""),
			)
		}
	}

	pub fn get_clicked_element(&mut self) -> Option<&mut MenuElement<B>> {
		self.elements.iter_mut().find(|e| e.clicked)
	}

	pub fn get_selected_element(&mut self) -> Option<&mut MenuElement<B>> {
		self.elements.iter_mut().find(|e| if let MenuKind::EditField { selected, .. } = e.kind { selected } else { false })
	}

	pub fn get_element_by_name(&mut self, name: &'static str) -> Option<&mut MenuElement<B>> {
		self.elements.iter_mut().find(|e| e.name == name)
	}

}

impl<B: Backend> App<B> {
	pub fn tick_menu(&mut self, runnable: &mut Runnable<B>) {
		let mut opt_on_click = None;
		if self.peripherals_state.key_pressed(&Key::LeftMouse) {
			for element in &mut self.menu.elements {
				element.clicked = element.is_colliding(self.cursor_position);
			}
			if let Some(elem) = self.menu.get_selected_element() {
				if let MenuKind::EditField { selected, .. } = &mut elem.kind {
					*selected = false;
				}
			}
		} else if let Some(element) = self.menu.get_clicked_element() {
			element.clicked = false;
			match &mut element.kind {
				MenuKind::Button { on_click, .. } => {
					opt_on_click = Some(on_click.clone());
				}
				MenuKind::EditField { selected, .. } => {
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
	}

	pub fn draw_menu(&mut self) {
		let mut draw = Draw::new();
		draw.set_clear_color(Color::BLACK);

		// draw elements
		for element in &self.menu.elements {
			element.draw(&mut draw, self.cursor_position)
		}

		// draw cursor
		draw.rectangle(self.cursor_position, self.cursor_position + CanvasVec::new(0.01, 0.01), Color::RED);

		self.graphics_backend.draw(draw, None);
	}
}
