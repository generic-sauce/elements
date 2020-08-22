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

	pub fn draw(&mut self, index: usize, context: &DrawContext) {
		// character
		self.animation.draw(index, self.texture_center(), self.texture_radius(), context);

		// cursor
		context.draw_circle(self.center_position() + self.cursor, CURSOR_INDICATOR_RADIUS, Color::BLACK);

		// health
		self.draw_health(context);
	}

	pub fn draw_health(&mut self, context: &DrawContext) {
		let mut size = GameVec::new(PLAYER_SIZE.x, TILESIZE / 4);
		let offset = GameVec::new(0, PLAYER_SIZE.y / 2 + TILESIZE);
		context.draw_sprite(self.center_position() + offset, size, Color::BLACK, None, Flip::Normal);
		size.x = (size.x as f32 * (self.health as f32 / MAX_HEALTH as f32)) as i32;
		context.draw_sprite(self.center_position() + offset, size, Color::GREEN, None, Flip::Normal);
	}
}
