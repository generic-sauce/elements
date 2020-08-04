use crate::prelude::*;

use crate::texture_state::TextureState;
use super::TileMap;
use super::Tile;

impl TileMap {
    pub fn render(&mut self, w: &mut RenderWindow, texture_state: &TextureState) {
        for (index, tile) in self.tiles.iter().enumerate() {
            let index = index as u32;
            let pos = Vector2f::new((index % self.size.x) as f32, (index / self.size.x) as f32);

            let mut shape = RectangleShape::new();
            shape.set_size(Vector2f::new(1.0, 1.0));
            shape.set_origin(Vector2f::new(0.5, 0.5));
            shape.set_position(pos);
            let color = match tile {
                Tile::Void => Color::rgb(32, 32, 32),
                Tile::Ground => Color::rgb(64, 64, 64),
            };
            shape.set_fill_color(color);

            let size = Vector2f::new(w.size().x as f32, w.size().y as f32);
            // let ratio = size.x / size.y;
            let height = 64 as f32;
            let tile = size.y / height;
            shape.set_scale(Vector2f::new(tile, tile));
            shape.set_position(shape.position() * Vector2f::new(tile, -tile) + size / 2.0);
            w.draw_rectangle_shape(&shape, RenderStates::default());
        }
    }
}
