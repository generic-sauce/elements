use crate::prelude::*;

impl TileMap {
    pub fn draw(&mut self, context: &Context) {
        context.draw_tilemap(self);
    }
}
