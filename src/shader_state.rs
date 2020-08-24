use crate::prelude::*;

macro_rules! setup {
	($($x:ident : ($y:expr, $z:expr, $w: expr)),*) => {

		#[derive(Copy, Clone, Debug)]
		#[repr(usize)]
		pub enum ShaderId {
			$($x),*
		}

		impl ShaderState {
			pub fn new() -> ShaderState {
				let mut shaders = Vec::new();
				$(
					shaders.push(Shader::from_file($y, $z, $w).expect("could not load file"));
				)*
				ShaderState { shaders }
			}
		}
	};
}

pub struct ShaderState {
	shaders: Vec<Shader<'static>>,
}

impl ShaderState {
	pub fn get_shader(&mut self, id: ShaderId) -> &'_ mut Shader<'static> {
		&mut self.shaders[id as usize]
	}
}

setup!(
	Fluid: (Some("res/vertex.glsl"), None, Some("res/fluids_fragment.glsl")),
	Tilemap: (Some("res/vertex.glsl"), None, Some("res/tilemap_fragment.glsl")),
	Noise: (Some("res/vertex.glsl"), None, Some("res/noise.glsl"))
);
