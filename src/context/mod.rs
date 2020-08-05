use crate::prelude::*;

pub struct Context<'a> {
    window: &'a mut RenderWindow,
    texture_state: &'a TextureState,
    shader_state: &'a mut ShaderState,
    tilemap_size: Vec2u,
}

impl<'a> Context<'a> {
    pub fn new(window: &'a mut RenderWindow, texture_state: &'a TextureState, shader_state: &'a mut ShaderState, tilemap_size: Vec2u) -> Context<'a> {
        Context {
            window,
            texture_state,
            shader_state,
            tilemap_size,
        }
    }

    pub fn draw_sprite(&self, position: Vec2f, radius: Vec2f, color: Color, texture_id: Option<TextureId>) {
        let mut shape = RectangleShape::new();
        if let Some(texture_id) = texture_id {
            shape.set_texture(self.texture_state.get_texture(texture_id), true);
        }
        shape.set_size(radius * 2.0);
        shape.set_origin(radius);
        shape.set_position(position);
        shape.set_fill_color(color);

        let size = Vector2f::new(self.window.size().x as f32, self.window.size().y as f32);
        // let ratio = size.x / size.y;
        let height = self.tilemap_size.y as f32;
        let tile = size.y / height;
        shape.set_scale(Vector2f::new(tile, tile));
        shape.set_position(shape.position() * Vector2f::new(tile, -tile) + Vector2f::new(0.0, size.y));

        self.window.draw_rectangle_shape(&shape, RenderStates::default());
    }

    pub fn draw_fluids(&mut self) {
        let shader = &mut self.shader_state.get_shader(ShaderId::FluidShader);

        let n21 = |v: Vec2f| f32::fract(9923.236 * f32::fract(v.dot(Vec2f::new(293.42, 122.332))));
        let mut fluids = Vec::new();
        for y in 0..self.tilemap_size.y {
            for x in 0..self.tilemap_size.x {
                let n1 = n21(Vec2f::new(x as f32, y as f32));
                let n2 = n21(Vec2f::new(x as f32 + 10.0, y as f32 - 20.0));
                let n3 = n21(Vec2f::new(y as f32 + 10.0, x as f32 - 20.0));
                fluids.push((f32::sin(n2) * 255.0) as u8);
                fluids.push((f32::sin(n3) * 255.0) as u8);
                fluids.push(0 as u8);
                fluids.push((n1 < 0.3) as u8);
            }
        }

        let image = Image::create_from_pixels(self.tilemap_size.x, self.tilemap_size.y, &fluids).unwrap();
        let mut texture_sfbox: SfBox<Texture> = Texture::from_image(&image).unwrap();
        let x: *mut Texture = &mut *texture_sfbox;
        let texture: &'static mut Texture;
        unsafe { texture = &mut *x; }

        shader.set_uniform_texture("fluid_tex", texture);
        shader.set_uniform_vec2("fluid_tex_size", self.tilemap_size.to_f().into());

        let mut states = RenderStates::default();
        states.shader = Some(&shader);

        let size = self.window.size();
        let mut rect = RectangleShape::default();
        rect.set_texture(&texture, true);
        rect.set_size(Vector2f::new(size.x as f32, size.y as f32));
        self.window.draw_rectangle_shape(&rect, states);
    }
}
