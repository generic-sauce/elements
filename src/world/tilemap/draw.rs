use crate::prelude::*;

impl TileMap {
    pub fn draw(&mut self, context: &mut Context) {
        context.draw_tilemap(self);
    }
}
