use crate::prelude::*;
use super::*;

macro_rules! setup {
	($($id:ident : $resource:expr),*) => {

		#[derive(Copy, Clone, Debug)]
		#[repr(usize)]
		#[allow(unused)]
		pub enum TextureId {
			$($id),*
		}

		impl TextureId {
			#[allow(unused)]
			pub fn iter() -> impl Iterator<Item=TextureId> {
				[$(TextureId::$id),*].iter().cloned()
			}

			#[allow(unused)]
			pub fn filepath(self) -> String {
				match self {
					$(
						TextureId::$id => res($resource),
					)*
				}
			}
		}
	}
}

setup!(
	Unknown: "images/checkerboard.png",
	White: "images/white.png"
);

pub(super) fn create_texture_iter<'a>(device: &'a wgpu::Device, queue: &'a wgpu::Queue) -> impl Iterator<Item=wgpu::Texture> + 'a {
	TextureId::iter()
		.map(TextureId::filepath)
		.map(|filepath| image::open(filepath).unwrap().flipv().into_rgba())
		.map(move |image| {
			let size = image.dimensions();
			let size = Vec2u::new(size.0, size.1);
			let texture = create_texture(device, size);
			write_texture(queue, &texture, size, &image.as_raw()[..]);
			texture
		})
}

impl IntoTextureIndex for TextureId {
	fn into_texture_index(self) -> usize {
		self as usize
	}
}
