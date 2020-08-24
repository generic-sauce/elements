use crate::prelude::*;

use std::collections::HashMap;

pub struct Shader {
	pub inner_shader: SfmlShader<'static>,
	current_textures: HashMap<String, TextureContainer>,
}

pub enum TextureContainer {
	Boxed(SfBox<Texture>),
	Render(RenderTexture),
}

impl Shader {
	pub fn from_file(vertex: Option<&str>, geometry: Option<&str>, fragment: Option<&str>) -> Option<Shader> {
		SfmlShader::from_file(vertex, geometry, fragment).map(|inner_shader|
			Shader {
				inner_shader,
				current_textures: HashMap::new(),
			}
		)
	}

	pub fn set_uniform_texture(&mut self, name: &str, container: impl Into<TextureContainer>) {
		let container = container.into();
		self.current_textures.insert(name.to_string(), container);
		let t = self.current_textures.get(name)
			.unwrap()
			.texture();
        let x: *const Texture = t;
		let t: &'static Texture = unsafe { &*x };
		self.inner_shader.set_uniform_texture(name, t);
	}

	pub unsafe fn set_uniform_texture_raw<'a>(&mut self, name: &str, t: &'a Texture) {
		let x: *const Texture = t;
		let t: &'static Texture = unsafe { &*x };

		self.current_textures.remove(name);
		self.inner_shader.set_uniform_texture(name, t);
	}

	pub fn set_uniform_float(&mut self, name: &str, value: f32) {
		self.inner_shader.set_uniform_float(name, value);
	}

	pub fn set_uniform_vec2(&mut self, name: &str, value: sfml::graphics::glsl::Vec2) {
		self.inner_shader.set_uniform_vec2(name, value);
	}
}

impl TextureContainer {
	pub fn texture(&self) -> &'_ Texture {
		match self {
			TextureContainer::Boxed(x) => &**x,
			TextureContainer::Render(x) => x.texture(),
		}
	}
}
