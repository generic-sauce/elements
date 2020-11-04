mod menu_elements;

pub use menu_elements::*;
use crate::prelude::*;

// pub const DEFAULT_CURSOR_POSITION: CanvasVec = CanvasVec::new(0.5 * 16.0 / 9.0, 0.5);
pub const ASPECT_RATIO: f32 = 16.0 / 9.0;

pub struct Menu {
	pub elements: Vec<MenuElement>,
}

pub struct MenuRunnable {
	pub menu: Menu,
	pub next_runnable_change: RunnableChange,
}

impl Menu {
	pub fn main_menu() -> Menu {
		Menu {
			elements: vec!(
				MenuElement::new_button(CanvasVec::new(0.3 * ASPECT_RATIO, 0.6), CanvasVec::new(0.15, 0.05), "Best of 9", RunnableChange::Local(9)),
				MenuElement::new_button(CanvasVec::new(0.3 * ASPECT_RATIO, 0.4), CanvasVec::new(0.15, 0.05), "Best of 5", RunnableChange::Local(5)),
				MenuElement::new_button(CanvasVec::new(0.7 * ASPECT_RATIO, 0.6), CanvasVec::new(0.15, 0.05), "Infinite Game", RunnableChange::Local(0)),
				MenuElement::new_button(CanvasVec::new(0.7 * ASPECT_RATIO, 0.4), CanvasVec::new(0.15, 0.05), "Join Server", RunnableChange::Menu(MenuChoice::ConnectServer)),
				MenuElement::new_button(CanvasVec::new(0.85 * ASPECT_RATIO, 0.15), CanvasVec::new(0.15, 0.05), "Quit", RunnableChange::Quit),
			),
		}
	}

	pub fn connect_server_menu() -> Menu {
		Menu {
			elements: vec!(
				MenuElement::new_button(CanvasVec::new(0.5 * ASPECT_RATIO, 0.4), CanvasVec::new(0.15, 0.05), "Connect", RunnableChange::Client(String::from(""))),
				MenuElement::new_button(CanvasVec::new(0.15 * ASPECT_RATIO, 0.15), CanvasVec::new(0.15, 0.05), "Back", RunnableChange::Menu(MenuChoice::Main)),
				MenuElement::new_button(CanvasVec::new(0.85 * ASPECT_RATIO, 0.15), CanvasVec::new(0.15, 0.05), "Quit", RunnableChange::Quit),
				MenuElement::new_edit_field("ip", CanvasVec::new(0.5 * ASPECT_RATIO, 0.6), CanvasVec::new(0.15, 0.03), ""),
			)
		}
	}

	pub fn tick<B: Backend>(&mut self, app: &App<B>, next_runnable_change: &mut RunnableChange) {
		if app.peripherals_state.key_pressed(&Key::LeftMouse) {
			for element in &mut self.elements {
				element.clicked = element.is_colliding(&app.cursor_position);
			}
			if let Some(elem) = self.get_selected_element() {
				if let MenuKind::EditField { selected, .. } = &mut elem.kind {
					*selected = false;
				}
			}
		} else if let Some(element) = self.get_clicked_element() {
			element.clicked = false;
			match &mut element.kind {
				MenuKind::Button { runnable_change, .. } => {
					*next_runnable_change = runnable_change.clone();
					if let RunnableChange::Client(ip) = next_runnable_change {
						if let MenuKind::EditField { text, .. } = &self.get_element_by_name("ip").unwrap().kind {
							*ip = text.clone();
						}
					}
				}
				MenuKind::EditField { selected, .. } => {
					*selected = true;
				}
			}
		}
		if let Some(element) = self.get_selected_element() {
			element.apply_text(&app.peripherals_state.text);
			element.apply_key_events(&app.peripherals_state);
		}
	}

	pub fn get_clicked_element(&mut self) -> Option<&mut MenuElement> {
		self.elements.iter_mut().find(|e| e.clicked)
	}

	pub fn get_selected_element(&mut self) -> Option<&mut MenuElement> {
		self.elements.iter_mut().find(|e| if let MenuKind::EditField { selected, .. } = e.kind { selected } else { false })
	}

	pub fn get_element_by_name(&mut self, name: &'static str) -> Option<&mut MenuElement> {
		self.elements.iter_mut().find(|e| e.name == name)
	}
}

impl MenuRunnable {
	pub fn new(menu_choice: MenuChoice) -> MenuRunnable {
		let menu = match menu_choice {
			MenuChoice::Main => Menu::main_menu(),
			MenuChoice::ConnectServer => Menu::connect_server_menu(),
		};
		MenuRunnable {
			menu,
			next_runnable_change: RunnableChange::None,
		}
	}
}

impl<B: Backend> Runnable<B> for MenuRunnable {
	fn tick(&mut self, app: &mut App<B>) {
		let mouse_update = app.peripherals_state.cursor_move;
		app.cursor_position += mouse_update.cast() * 0.001 * (1.0, -1.0);
		app.cursor_position.y = app.cursor_position.y.max(0.0).min(1.0);
		app.cursor_position.x = app.cursor_position.x.max(0.0).min(ASPECT_RATIO);

		self.menu.tick(app, &mut self.next_runnable_change);
	}

	fn draw(&mut self, app: &mut App<B>, timed_loop_info: &TimedLoopInfo) {
		let mut draw = Draw::new(timed_loop_info.elapsed_time);

		draw.rectangle(CanvasVec::new(0.0, 0.0), CanvasVec::new(ASPECT_RATIO, 1.0), Color::BLACK);

		// draw elements
		for element in &self.menu.elements {
			element.draw(&mut draw, &app.cursor_position)
		}

		// draw cursor
        draw.rectangle(app.cursor_position, app.cursor_position + CanvasVec::new(0.01, 0.01), Color::RED);

		app.graphics_backend.draw(draw);
	}

	fn get_runnable_change(&mut self) -> RunnableChange {
		self.next_runnable_change.clone()
	}
}
