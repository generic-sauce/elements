use crate::prelude::*;

const MENU_BUTTONS_WIDTH: f32 = 0.1;
const MENU_BUTTONS_HEIGHT: f32 = 0.05;

impl<B: Backend> Menu<B> {
	pub fn new() -> Menu<B> {
		Menu {
			elements: Vec::new(),
			background: None,
		}
	}

	pub fn main_menu_items(selected: u8) -> Vec<MenuElement<B>> {
		let mut elements = vec![
			MenuElement::new_button(
				"main_online_button".to_string(),
				CanvasVec::new(MENU_BUTTONS_WIDTH, 1.0 - MENU_BUTTONS_HEIGHT),
				CanvasVec::new(MENU_BUTTONS_WIDTH, MENU_BUTTONS_HEIGHT),
				"Online".to_string(),
				Color::hex("153962"),
				MAIN_BUTTON_FONT_SIZE,
				Some(TextureId::Globe),
				Box::new(create_online_menu),
			),
			MenuElement::new_button(
				"main_local_button".to_string(),
				CanvasVec::new(MENU_BUTTONS_WIDTH, 1.0 - (MENU_BUTTONS_HEIGHT * 3.0)),
				CanvasVec::new(MENU_BUTTONS_WIDTH, MENU_BUTTONS_HEIGHT),
				"Local".to_string(),
				Color::hex("153962"),
				MAIN_BUTTON_FONT_SIZE,
				Some(TextureId::Gamepad),
				Box::new(create_local_menu)
			),
			MenuElement::new_button(
				"main_tutorial_button".to_string(),
				CanvasVec::new(MENU_BUTTONS_WIDTH, 1.0 - (MENU_BUTTONS_HEIGHT * 5.0)),
				CanvasVec::new(MENU_BUTTONS_WIDTH, MENU_BUTTONS_HEIGHT),
				"Tutorial".to_string(),
				Color::hex("153962"),
				MAIN_BUTTON_FONT_SIZE,
				Some(TextureId::Icon),
				Box::new(create_tutorial_menu)
			),
			MenuElement::new_button(
				"main_quit_button".to_string(),
				CanvasVec::new(MENU_BUTTONS_WIDTH, MENU_BUTTONS_HEIGHT),
				CanvasVec::new(MENU_BUTTONS_WIDTH, MENU_BUTTONS_HEIGHT),
				"Quit".to_string(),
				Color::hex("0c2542"),
				NORMAL_BUTTON_FONT_SIZE,
				None,
				Box::new(|_, _| std::process::exit(0))
			),
			MenuElement::new_label(
				"main_title".to_string(),
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

	pub fn online_menu(lobbies: &Vec<ShortLobbyInfo>) -> Menu<B> {
		let mut elements = Menu::main_menu_items(0);
		elements.extend(vec![
			MenuElement::new_label(
				"onlinemenu_title".to_string(),
				CanvasVec::new(0.5 * ASPECT_RATIO, 0.8),
				CanvasVec::new(0.15, 0.15),
				SUBTITLE_FONT_SIZE,
				"Online",
				TextAlign::Center,
			),
			/*
			MenuElement::new_button(
				"onlinemenu_play_button".to_string(),
				CanvasVec::new(0.5 * ASPECT_RATIO, 0.4),
				CanvasVec::new(0.15, 0.05),
				"Play",
				Color::hex("2f6f10"),
				GO_BUTTON_FONT_SIZE,
				None,
				Box::new(create_server_connector)
			),
			 */
			MenuElement::new_edit_field(
				"onlinemenu_playername".to_string(),
				CanvasVec::new(0.9 * ASPECT_RATIO, 0.95),
				CanvasVec::new(0.15, 0.022),
				DEFAULT_BUTTON_COLOR,
				"Your Name"
			),
			MenuElement::new_button(
				"onlinemenu_createlobby".to_string(),
				CanvasVec::new(0.8 * ASPECT_RATIO, 0.3),
				CanvasVec::new(0.25, 0.05),
				"Create Lobby".to_string(),
				Color::hex("2f6f10"),
				GO_BUTTON_FONT_SIZE,
				None,
				Box::new(move |app: &mut App<B>, _runnable: &mut Runnable<B>| {
					app.master_socket.send(&MasterServerPacket::CreateLobby("LobbyName".to_string())).unwrap(); // TODO add edit field
				} ),
			),
			MenuElement::new_button(
				"onlinemenu_refreshlobby".to_string(),
				CanvasVec::new(0.2 * ASPECT_RATIO, 0.3),
				CanvasVec::new(0.25, 0.05),
				"Refresh".to_string(),
				Color::hex("2f6f10"),
				GO_BUTTON_FONT_SIZE,
				None,
				Box::new(move |_app: &mut App<B>, runnable: &mut Runnable<B>| {
					if let Runnable::OnlineMenu(online_menu) = runnable {
						online_menu.lobbies.clear();
						online_menu.should_send_lobby_list_request = true;
					}
				} ),
			),
			/*
			MenuElement::new_label(
				"onlinemenu_description".to_string(),
				CanvasVec::new(0.5 * ASPECT_RATIO, 0.2),
				CanvasVec::new(0.15, 0.15),
				EXPLANATION_FONT_SIZE,
				"Play online against other players. You need an internet connection for this :D",
				TextAlign::Center,
			),
			 */
		]);

		for (i, lobby) in lobbies.iter().enumerate() {
			let lobby_id = lobby.lobby_id;
			let element = MenuElement::new_button(
				format!("onlinemenu_lobby{}", i),
				CanvasVec::new(0.5 * ASPECT_RATIO, 0.7-(i as f32)*0.08),
				CanvasVec::new(0.25, 0.035),
				lobby.name.clone(),
				Color::hex("006699"),
				LOBBY_BUTTON_FONT_SIZE,
				None,
				Box::new(move |app: &mut App<B>, _runnable: &mut Runnable<B>| {
					app.master_socket.send(&MasterServerPacket::JoinLobby(lobby_id)).unwrap();
				} ),
			);
			elements.push(element);
		}

		Menu {
			elements,
			background: Some(TextureId::SkyBackground),
		}
	}

	pub fn local_menu() -> Menu<B> {
		let mut elements = Menu::main_menu_items(1);
		elements.extend(vec![
			MenuElement::new_label(
				"localmenu_title".to_string(),
				CanvasVec::new(0.5 * ASPECT_RATIO, 0.8),
				CanvasVec::new(0.15, 0.15),
				SUBTITLE_FONT_SIZE,
				"Local",
				TextAlign::Center,
			),
			MenuElement::new_button(
				"localmenu_play_button".to_string(),
				CanvasVec::new(0.5 * ASPECT_RATIO, 0.4),
				CanvasVec::new(0.15, 0.05),
				"Play".to_string(),
				Color::hex("2f6f10"),
				GO_BUTTON_FONT_SIZE,
				None,
				Box::new(create_local(5)),
			),
			MenuElement::new_label(
				"localmenu_description".to_string(),
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
		}
	}

	pub fn tutorial_menu() -> Menu<B> {
		let mut elements = Menu::main_menu_items(2);
		elements.extend(vec![
			MenuElement::new_image(
				"tutorial_controller_image".to_string(),
				ViewVec::new(0.5, 0.5).to_canvas(),
				CanvasVec::new(16.0 / 9.0 * 0.666, 0.666),
				TextureId::Controls,
			),
			MenuElement::new_label(
				"tutorial_title".to_string(),
				CanvasVec::new(0.5 * ASPECT_RATIO, 0.8),
				CanvasVec::new(0.15, 0.15),
				SUBTITLE_FONT_SIZE,
				"Tutorial",
				TextAlign::Center,
			),
			MenuElement::new_label(
				"tutorial_description1".to_string(),
				CanvasVec::new(0.5 * ASPECT_RATIO, 0.2),
				CanvasVec::new(0.15, 0.15),
				EXPLANATION_FONT_SIZE,
				"Fluids will follow the cursor and keep their momentum when thrown.",
				TextAlign::Center,
			),
			MenuElement::new_label(
				"tutorial_description2".to_string(),
				CanvasVec::new(0.5 * ASPECT_RATIO, 0.17),
				CanvasVec::new(0.15, 0.15),
				EXPLANATION_FONT_SIZE,
				"Dodge enemy fluids or block them with walls. By the way you can walk on walls ;)",
				TextAlign::Center,
			),
			MenuElement::new_label(
				"tutorial_description3".to_string(),
				CanvasVec::new(0.5 * ASPECT_RATIO, 0.1),
				CanvasVec::new(0.15, 0.15),
				EXPLANATION_FONT_SIZE,
				"Learn the player controls by playing simple exercises. This currently not implemented :|",
				TextAlign::Center,
			),
		]);
		Menu {
			elements,
			background: Some(TextureId::SkyBackground),
		}
	}

	pub fn server_connector_menu() -> Menu<B> {
		unimplemented!()
	}

	pub fn in_game_menu(quit_action: OnEvent<B>) -> Menu<B> {
		Menu {
			elements: vec!(
				MenuElement::new_label(
					"ingame_game_paused_label".to_string(),
					CanvasVec::new(0.5 * ASPECT_RATIO, 0.8),
					CanvasVec::new(0.15, 0.15),
					SUBTITLE_FONT_SIZE,
					"Game Paused",
					TextAlign::Center,
				),
				MenuElement::new_button(
					"ingame_resume_button".to_string(),
					CanvasVec::new(0.5 * ASPECT_RATIO, 0.6),
					CanvasVec::new(0.15, 0.05),
					"Resume".to_string(),
					Color::hex("2f6f10"),
					GO_BUTTON_FONT_SIZE,
					None,
					Box::new(unpause),
				),
				MenuElement::new_button(
					"ingame_quit_button".to_string(),
					CanvasVec::new(0.5 * ASPECT_RATIO, 0.4),
					CanvasVec::new(0.15, 0.05),
					"Quit".to_string(),
					Color::hex("b52f1c"),
					GO_BUTTON_FONT_SIZE,
					None,
					quit_action,
				),
			),
			background: None,
		}
	}
}
