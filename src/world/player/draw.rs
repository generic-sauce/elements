use crate::prelude::*;

const IMG_SIZE: f32 = 64.0;
const IMG_PLAYER_WIDTH: f32 = 22.0;
const IMG_PLAYER_HEIGHT: f32 = 55.0;

const FACTOR_X: f32 = IMG_SIZE / IMG_PLAYER_WIDTH;
const FACTOR_Y: f32 = IMG_SIZE / IMG_PLAYER_HEIGHT;

impl Player {
	// the texture rect has been obtained by reading the .png file

	fn texture_center(&self) -> Vec2f {
		Vec2f::new(
			(self.left_bot.x + PLAYER_SIZE.x / 2) as f32 / TILESIZE as f32,
			(self.left_bot.y as f32 / TILESIZE as f32)
				+ (PLAYER_SIZE.y as f32 / 2.0 * FACTOR_Y) / TILESIZE as f32,
		)
	}

	fn texture_radius(&self) -> Vec2f {
		(PLAYER_SIZE.to_f() / TILESIZE as f32) / 2.0 * Vec2f::new(FACTOR_X, FACTOR_Y)
	}

	pub fn draw(&mut self, context: &Context) {
		context.draw_sprite(self.texture_center(), self.texture_radius(), Color::WHITE, Some(TextureId::PlayerIdle1));
	}
}
