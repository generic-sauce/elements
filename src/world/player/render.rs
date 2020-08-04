use sfml::system::{Vector2f};
use sfml::graphics::{RenderWindow, Color, RenderStates, RenderTarget, Shape, RectangleShape, Texture, Transformable, Transform};
use sfml::window::{Style, VideoMode, Event, Key};

use crate::texture_state::{TextureId, TextureState};
use super::Player;

impl Player {
    pub fn render(&mut self, w: &mut RenderWindow, texture_state: &TextureState) {
        let mut shape = RectangleShape::with_texture(texture_state.get_texture(TextureId::PlayerIdle1));
        shape.set_size(self.size * 2.0);
        shape.set_origin(self.size);
        shape.set_position(self.position);

        let size = Vector2f::new(w.size().x as f32, w.size().y as f32);
        let ratio = size.x / size.y;
        let height = 32 as f32;
        let tile = size.y / height;
        shape.set_scale(Vector2f::new(tile, tile));
        shape.set_position(shape.position() * Vector2f::new(1.0, -1.0) + size / 2.0);
        w.draw_rectangle_shape(&shape, RenderStates::default());
    }
}
