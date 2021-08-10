use crate::prelude::*;

pub struct OnlineMenu<B: Backend> {
	pub lobbies: Vec<ShortLobbyInfo>,
	pub should_send_lobby_list_request: bool,
	pub should_set_username_from_storage: bool,
	_p: PhantomData<B>,
}

impl<B: Backend> OnlineMenu<B> {
	pub fn new() -> OnlineMenu<B> {

		OnlineMenu {
			lobbies: Vec::new(),
			should_send_lobby_list_request: true,
			should_set_username_from_storage: true,
			_p: PhantomData,
		}
	}

	pub fn tick(&mut self, app: &mut App<B>, packets: Vec<MasterClientPacket>) -> Option<LongLobbyInfo>{
		self.tick_username_field(app);

		if self.should_send_lobby_list_request && app.master_socket.is_open() {
			app.master_socket.send(&MasterServerPacket::LobbyListRequest).expect("Could not send lobby list request");
			self.should_send_lobby_list_request = false;
		}

		let mut opt_lobby_info = None;

		for p in packets {
			match p {
				MasterClientPacket::LobbyListResponse(lobby_infos) => self.lobbies = lobby_infos,
				MasterClientPacket::LobbyInfoUpdate(lobby_info) => opt_lobby_info = Some(lobby_info),
				_ => eprintln!("WARN: Got invalid packet from master server: {:?}", p),
			}
		};

		opt_lobby_info
	}

	fn tick_username_field(&mut self, app: &mut App<B>) {
		if self.should_set_username_from_storage {
			app.menu_cache.edit_field.get_mut("onlinemenu_playername").unwrap().text = app.storage_backend.get("username").unwrap_or_else(String::new);
			self.should_set_username_from_storage = false;
		}

		let e = app.menu_cache.edit_field.get("onlinemenu_playername").unwrap();
		if app.storage_backend.get("username").unwrap_or_else(String::new) != e.text { // update name
			app.storage_backend.set("username", &e.text);
			app.should_send_login = true;
		}
	}

	pub fn build_menu(&self) -> Menu<B> {
		let lobbies = &self.lobbies[..];

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
			MenuElement::new_edit_field(
				"onlinemenu_createlobby_name".to_string(),
				CanvasVec::new(0.8 * ASPECT_RATIO, 0.4),
				CanvasVec::new(0.15, 0.022),
				DEFAULT_BUTTON_COLOR,
				"Lobby Name"
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
					if !app.master_socket.is_open() {
						eprintln!("can't open lobby, you have no connection to the master server!");
						return;
					}

					let new_lobby_name = &app.menu_cache.edit_field.get("onlinemenu_createlobby_name").unwrap().text;
					app.master_socket.send(&MasterServerPacket::CreateLobby(new_lobby_name.to_string())).unwrap();
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
					if !app.master_socket.is_open() {
						eprintln!("can't join lobby, you have no connection to the master server!");
						return;
					}

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

}
