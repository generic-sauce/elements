mod music;

use music::*;
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
	pub music_sender: Sender<MusicCommand>,
}

impl App {
	pub fn new() -> App {
		let context_settings = ContextSettings::default();
		let mut window = RenderWindow::new(VideoMode::desktop_mode(), "Elements 2", Style::DEFAULT, &context_settings);
		window.set_mouse_cursor_visible(false);

		let gilrs = gilrs::Gilrs::new().expect("Failed to create gilrs");

		let world = World::new();
		let tilemap_texture = create_tilemap_texture(&world.tilemap.tiles, world.tilemap.size);

		let (music_sender, music_receiver) = channel();

		thread::spawn(move || Musician::new(music_receiver).run() );

		App {
			window,
			world,
			tilemap_texture,
			texture_state: TextureState::new(),
			shader_state: ShaderState::new(),
			font_state: FontState::new(),
			animation_state: AnimationState::new(),
			gilrs,
			music_sender,
		}
	}

	fn apply_command(&mut self, c: Command) {
		match c {
			Command::UpdateTileMapTexture => {
				self.tilemap_texture = create_tilemap_texture(&self.world.tilemap.tiles, self.world.tilemap.size);
			},
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
	}

	#[allow(unused)]
	pub fn send_music_command(&mut self, c: MusicCommand) {
		self.music_sender.send(c).unwrap();
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
