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
	pub sound_manager: SoundManager,
	pub restart_state: RestartState,
}

impl App {
	pub fn new() -> App {
		let context_settings = ContextSettings::default();
		let mut window = RenderWindow::new(VideoMode::desktop_mode(), "Elements 2", Style::DEFAULT, &context_settings);
		window.set_mouse_cursor_visible(false);

		let gilrs = gilrs::Gilrs::new().expect("Failed to create gilrs");

		let world = World::new();
		let tilemap_texture = create_tilemap_texture(&world.tilemap.tiles, world.tilemap.size);

		App {
			window,
			world,
			tilemap_texture,
			texture_state: TextureState::new(),
			shader_state: ShaderState::new(),
			font_state: FontState::new(),
			animation_state: AnimationState::new(),
			gilrs,
			sound_manager: SoundManager::new(),
			restart_state: RestartState::Game,
		}
	}

	pub fn handle(&mut self, handler: &AppEventHandler) {
		if handler.tilemap_changed {
			self.tilemap_texture = create_tilemap_texture(&self.world.tilemap.tiles, self.world.tilemap.size);
		}
		if let Some(dmg) = (0..2).map(|p| handler.damages[p]).max() {
			if dmg > 0 {
				let volume = (dmg as f32 / 100.0).max(0.5).min(2.0);
				self.sound_manager.play_sound(SoundId::Whiz, volume);
			}
		}
	}

	pub fn tick(&mut self) {
		let mut handler = AppEventHandler::new();
		self.world.tick(&mut handler);
		self.handle(&handler);
		self.update_music();
		self.sound_manager.tick();
	}

	pub fn update_music(&mut self) {
		let mut critical_level = 0;
		for player in &self.world.players {
			if player.health < MAX_HEALTH / 2 {
				critical_level += 1;
			}
		}
		let sound_id = [SoundId::APart, SoundId::BPart, SoundId::DPart][critical_level];
		if self.sound_manager.current_music_id.map_or(true, |music_id| music_id != sound_id) {
			self.sound_manager.play_music(sound_id);
		}
	}
}

fn create_tilemap_texture(tiles: &Vec<Tile>, size: TileVec) -> SfBox<Texture> {
	let mut pixels = Vec::new();
	for &tile in tiles.iter() {
		let team: u8 = match tile {
			Tile::Wall { owner, .. } => owner as u8 * 255,
			Tile::Brick (Brick {owner, ..}) => owner as u8 * 255,
			_ => 0,
		};
		let ground: u8 = match tile {
			Tile::Void => 0,
			_ => 255,
		};
		let ratio: u8 = match tile {
			Tile::Wall { .. } => 255,
			Tile::Brick { .. } => 255,
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
