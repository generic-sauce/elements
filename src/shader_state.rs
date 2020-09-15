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
				use std::ops::Deref;

				let mut shaders = Vec::new();
				$(
					let y = $y.map(res);
					let z = $z.map(res);
					let w = $w.map(res);

					let y_ref: Option<&str> = y.as_ref().map(|x| x.deref());
					let z_ref: Option<&str> = z.as_ref().map(|x| x.deref());
					let w_ref: Option<&str> = w.as_ref().map(|x| x.deref());
					shaders.push(Shader::from_file(y_ref, z_ref, w_ref).expect("could not load file"));
				)*
				ShaderState { shaders }
			}
		}
	};
}

pub struct ShaderState {
	shaders: Vec<Shader>,
}

impl ShaderState {
	pub fn get_shader(&mut self, id: ShaderId) -> &'_ mut Shader {
		&mut self.shaders[id as usize]
	}
}

setup!(
	Fluid: (Some("shader/vertex.glsl"), None, Some("shader/fluids_fragment.glsl")),
	Tilemap: (Some("shader/vertex.glsl"), None, Some("shader/tilemap_fragment.glsl")),
	Noise: (Some("shader/vertex.glsl"), None, Some("shader/noise_fragment.glsl"))
);
