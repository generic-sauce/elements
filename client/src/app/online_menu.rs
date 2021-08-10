use crate::prelude::*;

pub struct OnlineMenu<B: Backend> {
	pub lobbies: Vec<ShortLobbyInfo>,
	pub should_send_lobby_list_request: bool,
	_p: PhantomData<B>,
}

impl<B: Backend> OnlineMenu<B> {
	pub fn new() -> OnlineMenu<B> {

		OnlineMenu {
			lobbies: Vec::new(),
			should_send_lobby_list_request: true,
			_p: PhantomData,
		}
	}

	pub fn tick(&mut self, app: &mut App<B>, packets: Vec<MasterClientPacket>) -> Option<LongLobbyInfo>{
		OnlineMenu::tick_username_field(app);

		if self.should_send_lobby_list_request {
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

	fn tick_username_field(app: &mut App<B>) {
		let e = app.menu_cache.edit_field.get("onlinemenu_playername").unwrap();
		if app.storage_backend.get("username").unwrap_or_else(String::new) != e.text {
			app.storage_backend.set("username", &e.text);
		}
	}
}
