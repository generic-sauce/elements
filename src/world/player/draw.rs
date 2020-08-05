use crate::prelude::*;

impl Player {
    pub fn draw(&mut self, context: &Context) {
        context.draw_sprite(self.left_bot.to_f() / TILESIZE as f32, PLAYER_SIZE.to_f() / TILESIZE as f32, Color::WHITE, Some(TextureId::PlayerIdle1));
    }
}
