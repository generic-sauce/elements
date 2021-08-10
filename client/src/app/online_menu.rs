use crate::prelude::*;

pub struct OnlineMenu<B: Backend> {
	pub lobbies: Vec<ShortLobbyInfo>,
	_p: PhantomData<B>,
}

impl<B: Backend> OnlineMenu<B> {
	pub fn new() -> OnlineMenu<B> {

		OnlineMenu {
			lobbies: Vec::new(),
			_p: PhantomData,
		}
	}

	pub fn tick(&mut self, app: &mut App<B>, packets: Vec<MasterClientPacket>) {
		OnlineMenu::tick_username_field(app);

		self.lobbies = vec![
			ShortLobbyInfo { lobby_id: 0, name: "my first lobby".to_string() },
			ShortLobbyInfo { lobby_id: 0, name: "lobby of doggy".to_string() },
			ShortLobbyInfo { lobby_id: 0, name: "best lobby".to_string() },
			ShortLobbyInfo { lobby_id: 0, name: "very long lobby name, probably will break".to_string() },
		];

		for p in packets {
			match p {
				MasterClientPacket::LobbyListResponse(lobby_infos) => { self.lobbies = lobby_infos; },
				MasterClientPacket::LobbyInfoUpdate(lobby_info) => { unimplemented!() }, // TODO
				_ => eprintln!("WARN: Got invalid packet from master server: {:?}", p),
			}
		}
	}

	fn tick_username_field(app: &mut App<B>) {
		let e = app.menu_cache.edit_field.get("onlinemenu_playername").unwrap();
		if app.storage_backend.get("username").unwrap_or_else(String::new) != e.text {
			app.storage_backend.set("username", &e.text);
		}
	}
}
