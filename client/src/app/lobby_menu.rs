use crate::prelude::*;

static AVAILABLE_MAPS: &'static[&'static str] = &["map01.png", "map02.png", "map03.png", "map04.png"];

pub struct LobbyMenu<B: Backend> {
	pub long_lobby_info: LongLobbyInfo,
	show_map_choose_menu: bool,
	pub tilemap_name: String,
	pub _p: PhantomData<B>,
}

impl<B: Backend> LobbyMenu<B> {
	pub fn from_lobby_info(long_lobby_info: LongLobbyInfo) -> LobbyMenu<B> {
		LobbyMenu {
			long_lobby_info,
			show_map_choose_menu: false,
			tilemap_name: String::from(AVAILABLE_MAPS[0]),
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

		// title
		elements.push(MenuElement::new_label(
			"lobbymenu_title".to_string(),
			CanvasVec::new(0.5 * ASPECT_RATIO, 0.87),
			CanvasVec::new(0.15, 0.15),
			0.07,
			&self.long_lobby_info.name,
			TextAlign::Left,
		));

		if !self.show_map_choose_menu {
			elements.push(MenuElement::new_label(
				"lobbymenu_mapname".to_string(),
				CanvasVec::new(0.8 * ASPECT_RATIO, 0.4),
				CanvasVec::new(0.1, 0.03),
				0.04,
				&format!("Map: {}", self.tilemap_name),
				TextAlign::Center,
			));
		}

		// choose map menu
		if self.show_map_choose_menu {
			elements.push(
				MenuElement::new_panel(
					"lobbymenu_changemappanel".to_string(),
					CanvasVec::new(0.5*ASPECT_RATIO, 0.5),
					CanvasVec::new(0.3*ASPECT_RATIO, 0.3),
					Color::rgb(0.1, 0.2, 0.4),
				)
			);

			// map list view
			let content = AVAILABLE_MAPS.iter().map(|m| vec![m.to_string()]).collect();
			let events = AVAILABLE_MAPS.iter().map(|m| Box::new(move |app: &mut App<B>, runnable: &mut Runnable<B>| {
				match runnable {
					Runnable::LobbyMenu(lm) => {
						lm.choose_map(m);
					},
					_ => panic!("lobbymenu_changemap button clicked, but runnable is not LobbyMenu"),
				}
				/*
				if let Some(s) = &mut app.master_socket {
					match s.send(&MasterServePacket::Change)
				}
				 */
			}) as OnEvent<B>).collect();
			let map_list_view = MenuElement::new_list_view_elements(
				"lobbymenu_maps".to_string(),
				CanvasVec::new(0.5 * ASPECT_RATIO, 0.5),
				CanvasVec::new(0.25 * ASPECT_RATIO, 0.25),
				vec![0.01],
				vec!["Maps".to_string()],
				content,
				Some(events),
				menu_cache,
			);
			elements.extend(map_list_view);
		} else {
			// players
			let content = self.long_lobby_info.player_names.iter().map(|n| vec![n.to_string()]).collect();
			let players_list_view = MenuElement::new_list_view_elements(
				"lobbymenu_playernames".to_string(),
				CanvasVec::new(0.25 * ASPECT_RATIO, 0.5),
				CanvasVec::new(0.2 * ASPECT_RATIO, 0.2),
				vec![0.01],
				vec!["Players".to_string()],
				content,
				None,
				menu_cache,
			);
			elements.extend(players_list_view);
		}

		// if you are the lobby owner
		if self.long_lobby_info.your_player_index == 0 {
			// start game button
			elements.push(MenuElement::new_button(
				"lobbymenu_start_game_button".to_string(),
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

			// change map button
			if !self.show_map_choose_menu {
				elements.push(MenuElement::new_button(
					"lobbymenu_changemap".to_string(),
					CanvasVec::new(0.8 * ASPECT_RATIO, 0.3),
					CanvasVec::new(0.1, 0.03),
					"Change Map".to_string(),
					Color::rgb(0.2, 0.4, 0.6),
					0.03,
					None,
					Box::new(|_, runnable: &mut Runnable<B>| {
						match runnable {
							Runnable::LobbyMenu(lm) => {
								lm.show_map_choose_menu = true;
							},
							_ => panic!("lobbymenu_changemap button clicked, but runnable is not LobbyMenu"),
						}
					}),
				));
			}
		}

		// leave button
		elements.push(MenuElement::new_button(
			"lobbymenu_leave_button".to_string(),
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

	fn choose_map(&mut self, map: &str) {
		self.show_map_choose_menu = false;
		self.tilemap_name = map.to_string();
	}
}
