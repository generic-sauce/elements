use crate::prelude::*;

pub struct App {
	pub window: RenderWindow,
	pub world: World,
	pub player_animations: [Animation; 2],
	pub player_directions: [PlayerDirection; 2],
	pub tilemap_texture: SfBox<Texture>,
	pub texture_state: TextureState,
	pub shader_state: ShaderState,
	pub font_state: FontState,
	pub animation_state: AnimationState,
	pub gilrs: gilrs::Gilrs,
	pub input_states: [InputState; 2],
}

#[must_use]
pub enum Command {
	UpdateTileMapTexture,
	ChangeAnimation { player_id: usize, animation_id: AnimationId, direction: Option<PlayerDirection> }
}

impl App {
	pub fn new() -> App {
		let context_settings = ContextSettings::default();
		let mut window = RenderWindow::new(VideoMode::desktop_mode(), "Elements 2", Style::DEFAULT, &context_settings);
		window.set_mouse_cursor_visible(false);

		let gilrs = gilrs::Gilrs::new().expect("Failed to create gilrs");

		let world = World::new();
		let tilemap_texture = App::create_tilemap_texture(&world.tilemap.tiles, world.tilemap.size);

		App {
			window,
			world,
			player_animations: [Animation::new(AnimationId::BluePlayerIdle), Animation::new(AnimationId::RedPlayerIdle)],
			player_directions: [PlayerDirection::Right, PlayerDirection::Left],
			tilemap_texture,
			texture_state: TextureState::new(),
			shader_state: ShaderState::new(),
			font_state: FontState::new(),
			animation_state: AnimationState::new(),
			gilrs,
			input_states: [InputState::new(), InputState::new()],
		}
	}

	fn apply_command(&mut self, c: Command) {
		match c {
			Command::UpdateTileMapTexture => {
				self.tilemap_texture = App::create_tilemap_texture(&self.world.tilemap.tiles, self.world.tilemap.size);
			},
			Command::ChangeAnimation { player_id, animation_id, direction, } => {
				if self.player_animations[player_id].animation_id != animation_id {
					self.player_animations[player_id] = Animation::new(animation_id);
				}
				self.player_directions[player_id] = direction.unwrap_or(self.player_directions[player_id]);
			},
		}
	}

	pub fn apply_commands(&mut self, v: Vec<Command>) {
		for x in v {
			self.apply_command(x);
		}
	}

	pub fn tick(&mut self) {
		let cmds = self.world.tick(&self.input_states);
		self.apply_commands(cmds);

		for x in &mut self.player_animations {
			x.tick(&self.animation_state);
		}
	}

	pub fn draw(&mut self, elapsed_time: Duration, fps: u32, load: f32) {
		let aspect_ratio = 16.0 / 9.0;
		let (window_view, view, view_pixel_size) = self.get_views(aspect_ratio);

		// declare render target
		let mut game_texture_target = RenderTexture::new(view_pixel_size.x, view_pixel_size.y, false).unwrap();
		let mut game_noise_target = RenderTexture::new(view_pixel_size.x, view_pixel_size.y, false).unwrap();

		game_texture_target.set_view(&view);
		game_noise_target.set_view(&view);
		self.window.set_view(&window_view);

		let window_size = self.window.size();
		let window_size = Vec2u::new(window_size.x, window_size.y);
		let mut context = DrawContext {
			window_size: window_size,
			texture_state: &self.texture_state,
			shader_state: &mut self.shader_state,
			font_state: &self.font_state,
			animation_state: &self.animation_state,
			tilemap_size: self.world.tilemap.size,
			elapsed_time,
			player_animations: &self.player_animations,
			player_directions: &self.player_directions,
			tilemap_texture: &self.tilemap_texture,
			aspect_ratio: aspect_ratio,
		};

		// draw game
		context.fill_canvas_with_color(&game_texture_target, Color::rgb(115, 128, 56));
		self.world.draw(&game_texture_target, &mut context);
		context.apply_noise(&game_noise_target, game_texture_target);
		context.fill_canvas_with_texture(&self.window, game_noise_target);

		// draw debug info
		let text_size = 0.030;
		context.draw_text(&self.window, CanvasVec::new(0.0, 1.0 - text_size * 0.0), text_size,
						  &format!("{} / {}", self.world.kills[0], self.world.kills[1]), Origin::LeftTop);

		context.draw_text(&self.window, CanvasVec::new(0.0, 1.0 - text_size * 2.0), text_size,
						  &format!("elapsed time: {}", elapsed_time.as_secs()), Origin::LeftTop);
		context.draw_text(&self.window, CanvasVec::new(0.0, 1.0 - text_size * 3.0), text_size,
						  &format!("fps: {}", fps as u32), Origin::LeftTop);
		context.draw_text(&self.window, CanvasVec::new(0.0, 1.0 - text_size * 4.0), text_size,
						  &format!("load: {:.2}%", load * 100.0), Origin::LeftTop);

		let (fluid_count_0, fluid_count_1) = (
			self.world.fluidmap.iter()
				.filter(|x| x.owner == 0)
				.count(),
			self.world.fluidmap.iter()
				.filter(|x| x.owner == 1)
				.count()
		);
		context.draw_text(&self.window, CanvasVec::new(0.0, 1.0 - text_size * 5.0), text_size,
						  &format!("fluid count: {} / {}", fluid_count_0, fluid_count_1), Origin::LeftTop);

		self.window.display();
        self.window.clear(Color::BLACK);
	}

	fn get_views(&self, aspect_ratio: f32) -> (SfBox<View>, SfBox<View>, Vec2u) {
		let window_size = self.window.size();
		let window_size = Vector2f::new(window_size.x as f32, window_size.y as f32);
		let window_aspect_ratio = window_size.x / window_size.y;
		let aspect_factor = window_aspect_ratio / aspect_ratio;

		let wider_factor = f32::max(0.0, aspect_factor - 1.0) / 2.0;
		let higher_factor = f32::max(0.0, 1.0 / aspect_factor - 1.0) / 2.0;

		let left = -wider_factor * aspect_ratio;
		let width = aspect_ratio * (1.0 + 2.0 * wider_factor);
		let top = -higher_factor;
		let height = 1.0 + 2.0 * higher_factor;
		let window_view = View::from_rect(&FloatRect::new(left, 1.0 - top, width, -height));

		let wider_factor = wider_factor * 2.0 + 1.0;
		let higher_factor = higher_factor * 2.0 + 1.0;

		let view = View::from_rect(&FloatRect::new(0.0, 1.0, aspect_ratio, -1.0));
		let view_pixel_size = Vec2u::new(
			(window_size.x as f32 / wider_factor) as u32,
			(window_size.y as f32 / higher_factor) as u32
		);

		(window_view, view, view_pixel_size)
	}
}