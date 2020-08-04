use crate::prelude::*;

use crate::texture_state::TextureState;
use super::TileMap;
use super::Tile;
use crate::context::Context;

impl TileMap {
    pub fn draw(&mut self, context: &Context) {
        for (index, tile) in self.tiles.iter().enumerate() {
            let index = index as u32;
            let position = Vec2f::new((index % self.size.x) as f32, (index / self.size.x) as f32);

            let color = match tile {
                Tile::Void => Color::rgb(32, 32, 32),
                Tile::Ground => Color::rgb(64, 64, 64),
            };
            context.draw_sprite(position, Vec2f::new(0.5, 0.5), color, None);
        }
    }
}
