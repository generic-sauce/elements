mod sound;

pub use sound::*;
use crate::prelude::*;

pub struct App {
	pub window: RenderWindow,
	pub world: World,
	pub tilemap_texture: SfBox<Texture>,
	pub texture_state: TextureState,
	pub shader_state: ShaderState,
	pub font_state: FontState,
	pub animation_state: AnimationState,
	pub gilrs: gilrs::Gilrs,
	pub sound_sender: Sender<SoundCommand>,
	pub current_sound_id: SoundId,
}

impl App {
	pub fn new() -> App {
		let context_settings = ContextSettings::default();
		let mut window = RenderWindow::new(VideoMode::desktop_mode(), "Elements 2", Style::DEFAULT, &context_settings);
		window.set_mouse_cursor_visible(false);

		let gilrs = gilrs::Gilrs::new().expect("Failed to create gilrs");

		let world = World::new();
		let tilemap_texture = create_tilemap_texture(&world.tilemap.tiles, world.tilemap.size);

		let (sound_sender, sound_receiver) = channel();

		thread::spawn(move || SoundManager::new(sound_receiver).run() );

		App {
			window,
			world,
			tilemap_texture,
			texture_state: TextureState::new(),
			shader_state: ShaderState::new(),
			font_state: FontState::new(),
			animation_state: AnimationState::new(),
			gilrs,
			sound_sender,
			current_sound_id: SoundId::APart,
		}
	}

	fn apply_command(&mut self, c: Command) {
		let mut player_damaged = false;
		match c {
			Command::UpdateTileMapTexture => {
				self.tilemap_texture = create_tilemap_texture(&self.world.tilemap.tiles, self.world.tilemap.size);
			},
			Command::PlayerDamage { .. } => {
				player_damaged = true;
			}
		}
		if player_damaged {
			self.send_sound_command(SoundCommand::PlaySound(SoundId::Whiz));
		}
	}

	pub fn apply_commands(&mut self, v: Vec<Command>) {
		for x in v {
			self.apply_command(x);
		}
	}

	pub fn tick(&mut self) {
		let cmds = self.world.tick();
		self.apply_commands(cmds);
		self.update_music();
	}

	pub fn update_music(&mut self) {
		let mut critical_level = 0;
		for player in &self.world.players {
			if player.health < MAX_HEALTH / 2 {
				critical_level += 1;
			}
		}
		let sound_id = [SoundId::APart, SoundId::BPart, SoundId::DPart][critical_level];
		if sound_id != self.current_sound_id {
			self.send_sound_command(SoundCommand::PlayMusic(sound_id));
			self.current_sound_id = sound_id;
		}
	}

	#[allow(unused)]
	pub fn send_sound_command(&mut self, c: SoundCommand) {
		self.sound_sender.send(c).unwrap();
	}
}

fn create_tilemap_texture(tiles: &Vec<Tile>, size: TileVec) -> SfBox<Texture> {
	let mut pixels = Vec::new();
	for &tile in tiles.iter() {
		let team: u8 = match tile {
			Tile::Wall { owner, .. } => owner as u8 * 255, // TODO maybe owner should be u8 generally
			_ => 0,
		};
		let ground: u8 = match tile {
			Tile::Void => 0,
			_ => 255,
		};
		let ratio: u8 = match tile {
			Tile::Wall { .. } => 255, // TODO correct?
			_ => 0,
		};

		pixels.push(ground);
		pixels.push(team);
		pixels.push(ratio);
		pixels.push(255 as u8);
	}

	let image = Image::create_from_pixels(size.x as u32, size.y as u32, &pixels).unwrap();
	Texture::from_image(&image).unwrap()
}
