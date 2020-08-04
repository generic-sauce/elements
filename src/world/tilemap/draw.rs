use crate::prelude::*;

impl TileMap {
    pub fn draw(&mut self, context: &Context) {
        for (index, tile) in self.tiles.iter().enumerate() {
            let index = index as u32;
            let position = Vec2f::new((index % self.size.x) as f32, (index / self.size.x) as f32) + Vec2f::new(0.5, 0.5);

            let color = match tile {
                Tile::Void => Color::rgb(115, 158, 65),
                Tile::Ground => Color::rgb(51, 26, 26),
            };
            context.draw_sprite(position, Vec2f::new(0.5, 0.5), color, None);
        }
    }
}
