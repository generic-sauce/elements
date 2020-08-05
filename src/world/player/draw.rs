use crate::prelude::*;

impl Player {
    pub fn draw(&mut self, context: &Context) {
		let center = (self.left_bot.to_f() + (PLAYER_SIZE.to_f() / 2.0)) / TILESIZE as f32;
		let size = PLAYER_SIZE.to_f() / TILESIZE as f32;

        context.draw_sprite(center, size, Color::WHITE, Some(TextureId::PlayerIdle1));
    }
}
