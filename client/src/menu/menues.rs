use crate::prelude::*;

const MENU_BUTTONS_WIDTH: f32 = 0.1;
const MENU_BUTTONS_HEIGHT: f32 = 0.05;

impl<B: Backend> Menu<B> {
	pub fn new() -> Menu<B> {
		Menu {
			elements: Vec::new(),
			background: None,
			kind: MenuMode::Normal,
		}
	}

	pub fn main_menu_items(selected: u8) -> Vec<MenuElement<B>> {
		let mut elements = vec![
			MenuElement::new_button(
				CanvasVec::new(MENU_BUTTONS_WIDTH, 1.0 - MENU_BUTTONS_HEIGHT),
				CanvasVec::new(MENU_BUTTONS_WIDTH, MENU_BUTTONS_HEIGHT),
				"Online",
				Color::hex("153962"),
				MAIN_BUTTON_FONT_SIZE,
				Some(TextureId::Icon),
				Box::new(create_online_menu),
			),
			MenuElement::new_button(
				CanvasVec::new(MENU_BUTTONS_WIDTH, 1.0 - (MENU_BUTTONS_HEIGHT * 3.0)),
				CanvasVec::new(MENU_BUTTONS_WIDTH, MENU_BUTTONS_HEIGHT),
				"Local",
				Color::hex("153962"),
				MAIN_BUTTON_FONT_SIZE,
				Some(TextureId::Icon),
				Box::new(create_local_menu)
			),
			MenuElement::new_button(
				CanvasVec::new(MENU_BUTTONS_WIDTH, 1.0 - (MENU_BUTTONS_HEIGHT * 5.0)),
				CanvasVec::new(MENU_BUTTONS_WIDTH, MENU_BUTTONS_HEIGHT),
				"Tutorial",
				Color::hex("153962"),
				MAIN_BUTTON_FONT_SIZE,
				Some(TextureId::Icon),
				Box::new(create_tutorial_menu)
			),
			MenuElement::new_button(
				CanvasVec::new(MENU_BUTTONS_WIDTH, MENU_BUTTONS_HEIGHT),
				CanvasVec::new(MENU_BUTTONS_WIDTH, MENU_BUTTONS_HEIGHT),
				"Quit",
				Color::hex("0c2542"),
				NORMAL_BUTTON_FONT_SIZE,
				None,
				Box::new(|_, _| std::process::exit(0))
			),
			MenuElement::new_label(
				CanvasVec::new(0.5 * ASPECT_RATIO, 0.9),
				CanvasVec::new(0.15, 0.15),
				0.1,
				"Elements",
				TextAlign::Center,
			),
		];
		elements[selected as usize].color = Color::hex("295e9a");
		elements
	}

	pub fn online_menu() -> Menu<B> {
		let mut elements = Menu::main_menu_items(0);
		elements.extend(vec![
			MenuElement::new_label(
				CanvasVec::new(0.5 * ASPECT_RATIO, 0.8),
				CanvasVec::new(0.15, 0.15),
				SUBTITLE_FONT_SIZE,
				"Online",
				TextAlign::Center,
			),
			MenuElement::new_button(
				CanvasVec::new(0.5 * ASPECT_RATIO, 0.4),
				CanvasVec::new(0.15, 0.05),
				"Play",
				Color::hex("2f6f10"),
				GO_BUTTON_FONT_SIZE,
				None,
				Box::new(create_server_connector)
			),
			MenuElement::new_edit_field(
				"player_name",
				CanvasVec::new(0.9 * ASPECT_RATIO, 0.95),
				CanvasVec::new(0.15, 0.022),
				"",
				DEFAULT_BUTTON_COLOR,
				"Your Name"
			),
			MenuElement::new_label(
				CanvasVec::new(0.5 * ASPECT_RATIO, 0.2),
				CanvasVec::new(0.15, 0.15),
				EXPLANATION_FONT_SIZE,
				"Play online against other players. You need an internet connection for this :D",
				TextAlign::Center,
			),
		]);
		Menu {
			elements,
			background: Some(TextureId::SkyBackground),
			kind: MenuMode::Normal,
		}
	}

	pub fn local_menu() -> Menu<B> {
		let mut elements = Menu::main_menu_items(1);
		elements.extend(vec![
			MenuElement::new_label(
				CanvasVec::new(0.5 * ASPECT_RATIO, 0.8),
				CanvasVec::new(0.15, 0.15),
				SUBTITLE_FONT_SIZE,
				"Local",
				TextAlign::Center,
			),
			MenuElement::new_button(
				CanvasVec::new(0.5 * ASPECT_RATIO, 0.4),
				CanvasVec::new(0.15, 0.05),
				"Play",
				Color::hex("2f6f10"),
				GO_BUTTON_FONT_SIZE,
				None,
				Box::new(create_local(5)),
			),
			MenuElement::new_label(
				CanvasVec::new(0.5 * ASPECT_RATIO, 0.2),
				CanvasVec::new(0.15, 0.15),
				EXPLANATION_FONT_SIZE,
				"Play local with friends on the same computer :)",
				TextAlign::Center,
			),
		]);
		Menu {
			elements,
			background: Some(TextureId::SkyBackground),
			kind: MenuMode::Normal,
		}
	}

	pub fn tutorial_menu() -> Menu<B> {
		let mut elements = Menu::main_menu_items(2);
		elements.extend(vec![
			MenuElement::new_label(
				CanvasVec::new(0.5 * ASPECT_RATIO, 0.8),
				CanvasVec::new(0.15, 0.15),
				SUBTITLE_FONT_SIZE,
				"Tutorial",
				TextAlign::Center,
			),
			MenuElement::new_label(
				CanvasVec::new(0.5 * ASPECT_RATIO, 0.2),
				CanvasVec::new(0.15, 0.15),
				EXPLANATION_FONT_SIZE,
				"Learn the player controls by playing simple exercises. This currently not implemented :|",
				TextAlign::Center,
			),
		]);
		Menu {
			elements,
			background: Some(TextureId::SkyBackground),
			kind: MenuMode::Normal,
		}
	}

	pub fn server_connector_menu() -> Menu<B> {
		Menu {
			elements: vec!(
				MenuElement::new_button(
					CanvasVec::new(0.5 * ASPECT_RATIO, 0.25),
					CanvasVec::new(0.15, 0.05),
					"Cancel",
					Color::hex("b52f1c"),
					NORMAL_BUTTON_FONT_SIZE,
					None,
					Box::new(create_online_menu)
				),
				MenuElement::new_label(
					CanvasVec::new(0.5 * ASPECT_RATIO, 0.4),
					CanvasVec::new(0.15, 0.15),
					0.05,
					"Waiting for other player.",
					TextAlign::Center,
				),
			),
			background: Some(TextureId::SkyBackground),
			kind: MenuMode::Normal,
		}
	}

	pub fn in_game_menu(quit_action: OnEvent<B>) -> Menu<B> {
		Menu {
			elements: vec!(
				MenuElement::new_label(
					CanvasVec::new(0.5 * ASPECT_RATIO, 0.8),
					CanvasVec::new(0.15, 0.15),
					SUBTITLE_FONT_SIZE,
					"Game Paused",
					TextAlign::Center,
				),
				MenuElement::new_button(
					CanvasVec::new(0.5 * ASPECT_RATIO, 0.6),
					CanvasVec::new(0.15, 0.05),
					"Resume",
					Color::hex("2f6f10"),
					GO_BUTTON_FONT_SIZE,
					None,
					Box::new(unpause),
				),
				MenuElement::new_button(
					CanvasVec::new(0.5 * ASPECT_RATIO, 0.4),
					CanvasVec::new(0.15, 0.05),
					"Quit",
					Color::hex("b52f1c"),
					GO_BUTTON_FONT_SIZE,
					None,
					quit_action,
				),
			),
			background: None,
			kind: MenuMode::InGame { active: false },
		}
	}
}