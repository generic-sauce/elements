use crate::prelude::*;

impl Player {
    pub fn draw(&mut self, context: &Context) {
		// TODO calculate render-size of player!
		let center = (self.left_bot.to_f() + (PLAYER_SIZE.to_f() / 2.0)) / TILESIZE as f32;
		let radius = PLAYER_SIZE.to_f() / TILESIZE as f32 / 2.;

        context.draw_sprite(center, radius, Color::WHITE, Some(TextureId::PlayerIdle1));
    }
}
