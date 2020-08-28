use crate::prelude::*;

const IMG_SIZE: i32 = 64;
const IMG_PLAYER_WIDTH: i32 = 22;
const IMG_PLAYER_HEIGHT: i32 = 55;
const IMG_PLAYER_SIZE: GameVec = GameVec::new(IMG_PLAYER_WIDTH, IMG_PLAYER_HEIGHT);

impl Player {
	// the texture rect has been obtained by reading the .png file

	fn texture_center(&self) -> GameVec {
		GameVec::new(
			self.left_bot.x + PLAYER_SIZE.x / 2,
			self.left_bot.y + PLAYER_SIZE.y * IMG_SIZE / 2 / IMG_PLAYER_HEIGHT,
		)
	}

	fn texture_radius(&self) -> GameVec {
		PLAYER_SIZE * GameVec::new(IMG_SIZE, IMG_SIZE) / IMG_PLAYER_SIZE / 2
	}

	pub fn draw(&self, p: usize, target: &impl RenderTarget, context: &DrawContext) {
		// character
		let flip = if let PlayerDirection::Right = context.player_directions[p] { Flip::Normal } else { Flip::Horizontal };
		context.player_animations[p].draw(target, flip, self.texture_center(), self.texture_radius(), context);

		// cursor
		context.draw_circle(target, self.center_position() + self.cursor, CURSOR_INDICATOR_RADIUS, Color::BLACK);

		// health
		self.draw_health(target, context);
	}

	pub fn draw_health(&self, target: &impl RenderTarget, context: &DrawContext) {
		let mut size = GameVec::new(PLAYER_SIZE.x / 2, TILESIZE / 3);
		let offset = GameVec::new(0, PLAYER_SIZE.y + TILESIZE);
		let left_bot = self.left_bot + offset;
		context.draw_rect(target, left_bot, size, Color::BLACK, Origin::LeftBottom);
		size.x = (size.x as f32 * (self.health as f32 / MAX_HEALTH as f32)) as i32;
		context.draw_rect(target, left_bot, size, Color::GREEN, Origin::LeftBottom);
	}
}
