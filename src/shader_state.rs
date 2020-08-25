use crate::prelude::*;

use std::ops::Deref;

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
	shaders: Vec<Shader<'static>>,
}

impl ShaderState {
	pub fn get_shader(&mut self, id: ShaderId) -> &'_ mut Shader<'static> {
		&mut self.shaders[id as usize]
	}
}

setup!(
	Fluid: (Some("vertex.glsl"), None, Some("fluids_fragment.glsl")),
	Tilemap: (Some("vertex.glsl"), None, Some("tilemap_fragment.glsl"))
);
