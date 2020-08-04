use sfml::system::{Vector2f};
use sfml::graphics::{RenderWindow, Color, RenderStates, RenderTarget, Shape, RectangleShape, Texture, Transformable, Transform};

use crate::texture_state::{TextureId, TextureState};
use crate::world::player::Player;
use crate::world::tilemap::TileMap;

pub struct Context<'a> {
    window: &'a mut RenderWindow,
    texture_state: &'a TextureState,
}

impl<'a> Context<'a> {
    pub fn new(window: &'a mut RenderWindow, texture_state: &'a TextureState) -> Context<'a> {
        Context {
            window,
            texture_state,
        }
    }

    pub fn draw_sprite(&self, position: Vector2f, radius: Vector2f, color: Color, textureId: Option<TextureId>) {
        let mut shape = RectangleShape::new();
        if let Some(textureId) = textureId {
            shape.set_texture(self.texture_state.get_texture(textureId), true);
        }
        shape.set_size(radius * 2.0);
        shape.set_origin(radius);
        shape.set_position(position);
        shape.set_fill_color(color);

        let size = Vector2f::new(self.window.size().x as f32, self.window.size().y as f32);
        // let ratio = size.x / size.y;
        let height = 64 as f32;
        let tile = size.y / height;
        shape.set_scale(Vector2f::new(tile, tile));
        shape.set_position(shape.position() * Vector2f::new(tile, -tile) + size / 2.0);

        self.window.draw_rectangle_shape(&shape, RenderStates::default());
    }
}
