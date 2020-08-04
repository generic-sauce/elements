use sfml::system::Vector2f;
use sfml::graphics::{RenderWindow, Color, RenderStates, RenderTarget, Shape, RectangleShape, Texture, Transformable};
use sfml::window::{Style, VideoMode, Event, Key};

use crate::texture_state::{TextureId, TextureState};
use super::Player;

impl Player {
    pub fn render(&mut self, w: &mut RenderWindow, texture_state: &TextureState) {
        let mut shape = RectangleShape::with_texture(texture_state.get_texture(TextureId::PlayerIdle1));
        shape.set_position(self.position);
        shape.set_size(self.size * 2.0);
        shape.set_origin(self.size);
        w.draw_rectangle_shape(&shape, RenderStates::default());
    }
}
