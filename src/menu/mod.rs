mod menu_elements;

pub use menu_elements::*;
use crate::prelude::*;

pub const DEFAULT_CURSOR_POSITION: CanvasVec = CanvasVec::new(0.5 * 16.0 / 9.0, 0.5);
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

	pub fn tick(&mut self, app: &App, next_runnable_change: &mut RunnableChange) {
		if sfml::window::mouse::Button::Left.is_pressed() {
			for element in &mut self.elements {
				element.clicked = element.is_colliding(&app.cursor_position);
			}
			if let Some(elem) = self.get_selected_element() {
				if let MenuKind::EditField { selected, .. } = &mut elem.kind {
					*selected = false;
				}
			}
		} else {
			if let Some(element) = self.get_clicked_element() {
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

impl Runnable for MenuRunnable {
	fn tick(&mut self, app: &mut App) {
		let mouse_update: WindowVec = app.peripherals_state.cursor_move;
		app.cursor_position += CanvasVec::new(mouse_update.x, -mouse_update.y) * 0.001;
		app.cursor_position.y = app.cursor_position.y.max(0.0).min(1.0);
		app.cursor_position.x = app.cursor_position.x.max(0.0).min(ASPECT_RATIO);

		self.menu.tick(app, &mut self.next_runnable_change);

	}

	fn draw(&mut self, app: &mut App, timed_loop_info: &TimedLoopInfo) {
		/*
		app.window.display();
		app.window.clear(Color::BLACK);

		let aspect_ratio = 16.0 / 9.0;
		let (window_view, view, view_pixel_size) = get_views(app, aspect_ratio);

		let mut game_texture_target = RenderTexture::new(view_pixel_size.x as u32, view_pixel_size.y as u32, false).unwrap();
		game_texture_target.set_view(&view);
		app.window.set_view(&window_view);

		let window_size = app.window.size();
		let window_size = WindowVec::new(window_size.x as f32, window_size.y as f32);

		let mut context = DrawContext {
			window_size,
			texture_state: &app.texture_state,
			shader_state: &mut app.shader_state,
			font_state: &app.font_state,
			animation_state: &app.animation_state,
			elapsed_time: timed_loop_info.elapsed_time,
			aspect_ratio,
		};

		context.fill_canvas_with_color(&game_texture_target, Color::rgb(255, 0, 0));

		// draw elements
		for element in &self.menu.elements {
			element.draw(&app.window, &mut context, &app.cursor_position)
		}

		// draw cursor
		context.draw_circle(&app.window, app.cursor_position, 0.01, Color::BLACK);
		context.draw_circle(&app.window, app.cursor_position, 0.008, Color::WHITE);
		 */
	}

	fn get_runnable_change(&mut self) -> RunnableChange {
		self.next_runnable_change.clone()
	}
}