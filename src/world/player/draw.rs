use crate::prelude::*;

impl Player {
    pub fn draw(&mut self, context: &Context) {
        context.draw_sprite(self.position, self.size, Color::WHITE, Some(TextureId::PlayerIdle1));
    }
}
