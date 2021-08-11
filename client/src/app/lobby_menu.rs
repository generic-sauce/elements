use crate::prelude::*;

pub struct LobbyMenu<B: Backend> {
	pub long_lobby_info: LongLobbyInfo,
	pub _p: PhantomData<B>,
}

impl<B: Backend> LobbyMenu<B> {
	pub fn from_lobby_info(long_lobby_info: LongLobbyInfo) -> LobbyMenu<B> {
		LobbyMenu {
			long_lobby_info,
			_p: PhantomData,
		}
	}

	pub fn tick(&mut self, packets: Vec<MasterClientPacket>) -> Option<(String, u16)> {
		let mut opt_ret = None;

		for p in packets {
			match p {
				MasterClientPacket::LobbyInfoUpdate(x) => {
					self.long_lobby_info = x;
				},
				MasterClientPacket::GoToGameServer(domain, port) => {
					opt_ret = Some((domain, port));
				},
				MasterClientPacket::LobbyListResponse(_) => {}
			}
		}

		opt_ret
	}

	pub fn build_menu(&self, menu_cache: &MenuCache) -> Menu<B> {
		let mut elements = Vec::new();
		elements.push(MenuElement::new_label(
			"lobbymenu_title".to_string(),
			CanvasVec::new(0.5 * ASPECT_RATIO, 0.87),
			CanvasVec::new(0.15, 0.15),
			0.07,
			&self.long_lobby_info.name,
			TextAlign::Left,
		));
		let content = self.long_lobby_info.player_names.iter().map(|n| vec![n.to_string()]).collect();
		let list_view = MenuElement::new_list_view_elements(
			"lobbymenu_playernames".to_string(),
			CanvasVec::new(0.25 * ASPECT_RATIO, 0.5),
			CanvasVec::new(0.2 * ASPECT_RATIO, 0.2),
			vec![0.01],
			vec!["Players".to_string()],
			content,
			None,
			menu_cache,
		);

		elements.extend(list_view);

		if self.long_lobby_info.your_player_index == 0 { // if you are the lobby owner
			elements.push(MenuElement::new_button(
				"lobby_menu_start_game_button".to_string(),
				CanvasVec::new(0.8 * ASPECT_RATIO, 0.1),
				CanvasVec::new(0.15, 0.05),
				"Start Game".to_string(),
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
			CanvasVec::new(0.2 * ASPECT_RATIO, 0.1),
			CanvasVec::new(0.15, 0.05),
			"Leave".to_string(),
			Color::hex("b52f1c"),
			0.05,
			None,
			Box::new(|app: &mut App<B>, runnable: &mut Runnable<B>| {
				app.master_socket.send(&MasterServerPacket::LeaveLobby).unwrap();
				*runnable = Runnable::OnlineMenu(OnlineMenu::new());
			}),
		));


		Menu {
			elements,
			background: Some(TextureId::SkyBackground),
		}
	}
}
