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

	pub fn build_menu(&self) -> Menu<B> {
		let mut elements = Vec::new();
		elements.push(MenuElement::new_label(
			"lobbymenu_title".to_string(),
			CanvasVec::new(0.5 * ASPECT_RATIO, 0.87),
			CanvasVec::new(0.15, 0.15),
			0.07,
			&self.long_lobby_info.name,
			TextAlign::Left,
		));
		elements.push(MenuElement::new_label(
			"lobbymenu_playernames_title".to_string(),
			CanvasVec::new(0.25 * ASPECT_RATIO, 0.77),
			CanvasVec::new(0.15, 0.15),
			0.06,
			"Players:",
			TextAlign::Left,
		));
		// TODO
		for (i, name) in self.long_lobby_info.player_names.iter().enumerate() {
			elements.push(MenuElement::new_label(
				format!("lobbymenu_playername{}", i),
				CanvasVec::new(0.25 * ASPECT_RATIO, 0.67 - (i as f32) * 0.08),
				CanvasVec::new(0.15, 0.15),
				0.05,
				name,
				TextAlign::Left,
			));
		}

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
					if let Some(s) = &mut app.master_socket {
						if let Err(x) = s.send(&MasterServerPacket::StartGame) {
							eprintln!("can't send StartGame packet due to \"{}\"", x);
						}
					}
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
				if let Some(s) = &mut app.master_socket {
					if let Err(x) = s.send(&MasterServerPacket::LeaveLobby) {
						eprintln!("LobbyMenu: can't send LeaveLobby packet due to \"{}\"", x);
					} else {
						*runnable = Runnable::OnlineMenu(OnlineMenu::new());
					}
				} else {
					eprintln!("can't leave lobby: master socket is None");
				}
			}),
		));


		Menu {
			elements,
			background: Some(TextureId::SkyBackground),
		}
	}
}
