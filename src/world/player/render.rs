use sfml::graphics::{RenderWindow, Color, RenderTarget, RectangleShape, Texture};
use sfml::window::{Style, VideoMode, Event, Key};

use crate::texture_state::TextureState;
use super::Player;

impl Player {
    pub fn render(&mut self, w: &mut RenderWindow, texture_state: &TextureState) {
        // let mut shape : RectangleShape::with_texture();
    }
}
