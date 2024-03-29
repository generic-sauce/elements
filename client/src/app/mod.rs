mod runnable;
pub use runnable::*;

mod event_handler;
pub use event_handler::*;

mod online_menu;
mod lobby_menu;

pub use online_menu::*;
pub use lobby_menu::*;

use crate::prelude::*;

pub const DEFAULT_CURSOR_POSITION: CanvasVec = CanvasVec::new(0.5 * 16.0 / 9.0, 0.5);
pub const CURSOR_SPEED: f32 = 0.001;
const MUSIC_VOLUME: f32 = 0.2;
const WHIZ_VOLUME: f32 = 0.1;
const BONG_VOLUME: f32 = 0.1;
const END_SOUND_VOLUME: f32 = 0.3;

pub struct App<B: Backend> {
	pub input_backend: B::InputBackend,
	pub graphics_backend: B::GraphicsBackend,
	pub audio_backend: B::AudioBackend,
	pub storage_backend: B::StorageBackend,
	pub cursor_position: CanvasVec,
	pub peripherals_state: PeripheralsState,
	pub menu_cache: MenuCache,
	pub master_socket: Option<B::SocketBackend>, // used for communication with master server
	pub should_send_login: bool, // if set to true, you should send a MasterServerPacket::Login to the master server
}

// TODO automatically reconnect to master server

impl<B: Backend> App<B> {
	pub fn new(graphics_backend: B::GraphicsBackend, input_backend: B::InputBackend, storage_backend: B::StorageBackend, master_server_ip: &str) -> App<B> {
		let mut audio_backend = B::AudioBackend::new();
		audio_backend.set_music_volume(MUSIC_VOLUME);

		let master_socket = match B::SocketBackend::new(master_server_ip, DEFAULT_MASTER_SERVER_PORT) {
			Ok(x) => Some(x),
			Err(x) => {
				eprintln!("app: can't connect to master server due to \"{}\"", x);
				None
			}
		};

		App {
			input_backend,
			graphics_backend,
			audio_backend,
			storage_backend,
			cursor_position: DEFAULT_CURSOR_POSITION,
			peripherals_state: PeripheralsState::new(),
			menu_cache: MenuCache::new(),
			master_socket,
			should_send_login: true,
		}
	}

	pub fn fetch_peripherals(&mut self) {
		for ev in self.input_backend.events() {
			self.peripherals_state.update(&ev);
		}
	}

	fn update_cursor(&mut self) {
		let mouse_update = self.peripherals_state.cursor_move;
		self.cursor_position += mouse_update.cast() * CURSOR_SPEED * (1.0, -1.0);
		self.cursor_position.y = self.cursor_position.y.max(0.0).min(1.0);
		self.cursor_position.x = self.cursor_position.x.max(0.0).min(ASPECT_RATIO);
	}

	pub fn tick_draw(&mut self, runnable: &mut Runnable<B>) {
		if let Some(s) = &mut self.master_socket {
			if self.should_send_login {
				let username = self.storage_backend.get("username").unwrap_or_else(String::new);
				if let Err(x) = s.send(&MasterServerPacket::Login(username)) {
					eprintln!("can't login to master server due to \"{}\"", x);
				} else {
					self.should_send_login = false;
				}
			}
		}

		self.peripherals_state.reset();

		self.fetch_peripherals();
		self.input_backend.tick();
		self.update_cursor();

		let (menu, opt_on_click) = runnable.build_menu(self);
		if let Some(on_click) = opt_on_click {
			on_click(self, runnable);
		}

		self.tick_master_socket();

		runnable.tick(self);

		self.audio_backend.tick();

		self.check_game_over(runnable);

		let mut draw = Draw::new();
		runnable.draw(self, &mut draw);

		self.draw_menu(&menu, &mut draw, &runnable);

		self.graphics_backend.submit(draw);
	}

	fn tick_master_socket(&mut self) {
		if let Some(s) = &mut self.master_socket {
			s.tick();
		}
	}

	fn check_game_over(&mut self, runnable: &mut Runnable<B>) {
		// TODO: make this more generic. Runnable should be able to change runnable/menu
		if let Some(world) = runnable.get_world() {
			let winner_found = match world.is_game_over() {
				GameResult::None => false,
				GameResult::Winner(winner) => {
					println!("player {} won the match", winner);
					true
				}
				GameResult::Tie => {
					println!("match ended in a tie");
					true
				}
			};
			if winner_found {
				*runnable = Runnable::OnlineMenu(OnlineMenu::new());
			}
		}
	}

	fn handle(&mut self, handler: &AppEventHandler) {
		if let Some(dmg) = (0..2).map(|p| handler.damages[p]).max() {
			if dmg > 0 {
				let volume = (dmg as f32 / 100.0).max(0.5).min(2.0);
				self.audio_backend.play_sound(SoundId::Whiz, volume*WHIZ_VOLUME);
			}
		}

		if handler.new_game_started {
			self.audio_backend.play_sound(SoundId::Bong, BONG_VOLUME);
		}

		if handler.game_ended {
			self.audio_backend.play_sound(SoundId::End, END_SOUND_VOLUME);
		}
	}
}

pub fn tick_within_app<B: Backend>(world: &mut World, app: &mut App<B>) {
	let mut handler = AppEventHandler::new(world.players.len());
	world.tick(&mut handler);
	app.handle(&handler);

	update_music_within_app(world, app);
}

fn update_music_within_app<B: Backend>(world: &mut World, app: &mut App<B>) {
	let mut critical_level = 0;
	for player in &world.players {
		if player.health < MAX_HEALTH / 2 {
			critical_level += 1;
		}
	}
	let sound_id = [SoundId::APart, SoundId::BPart, SoundId::DPart][critical_level.min(2)];
	app.audio_backend.queue_music(sound_id);
}

pub fn apply_update_within_app<B: Backend>(world: &mut World, update: WorldUpdate, app: &mut App<B>) {
	let mut handler = AppEventHandler::new(world.players.len());
	world.apply_update(update, &mut handler);
	app.handle(&handler);
}
