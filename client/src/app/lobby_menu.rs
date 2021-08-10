use crate::prelude::*;

pub struct LobbyMenu<B: Backend> {
	pub long_lobby_info: LongLobbyInfo,
	_p: PhantomData<B>,
}

impl<B: Backend> LobbyMenu<B> {
	pub fn from_lobby_info(long_lobby_info: LongLobbyInfo) -> LobbyMenu<B> {
		LobbyMenu {
			long_lobby_info,
			_p: PhantomData,
		}
	}

	pub fn build_menu(&self) -> Menu<B> {
		let mut elements = Menu::main_menu_items(0);
		// TODO
		for (i, name) in self.long_lobby_info.player_names.iter().enumerate() {
			elements.push(MenuElement::new_label(
				format!("lobbymenu_playername{}", i),
				CanvasVec::new(0.5 * ASPECT_RATIO, 0.8 - (i as f32) * 0.2),
				CanvasVec::new(0.15, 0.15),
				0.05,
				name,
				TextAlign::Center,
			));
		}

		if self.long_lobby_info.your_player_index == 0 { // if you are the lobby owner
			elements.push(MenuElement::new_button(
				"lobby_menu_start_game_button".to_string(),
				CanvasVec::new(0.5 * ASPECT_RATIO, 0.4),
				CanvasVec::new(0.15, 0.05),
				"Start Game",
				Color::hex("2f6f10"),
				0.05,
				None,
				Box::new(|app: &mut App<B>, _| {
					app.master_socket.send(&MasterServerPacket::StartGame).unwrap();
				}),
			));
		}

		elements.push(MenuElement::new_button(
			"lobby_menu_leave_button".to_string(),
			CanvasVec::new(0.7 * ASPECT_RATIO, 0.4),
			CanvasVec::new(0.15, 0.05),
			"Leave",
			Color::hex("2f6f10"),
			0.05,
			None,
			Box::new(|app: &mut App<B>, _| {
				app.master_socket.send(&MasterServerPacket::LeaveLobby).unwrap();
			}),
		));


		Menu {
			elements,
			background: Some(TextureId::SkyBackground),
		}
	}
}
