mod texture;
use texture::*;

mod animation;
use animation::*;

mod misc;
use misc::*;

pub trait IntoTextureIndex {
	fn into_texture_index(self) -> usize;
}

pub(super) struct TextureState {
	textures: Vec<wgpu::Texture>,
	texture_views: Vec<wgpu::TextureView>,
}

impl TextureState {
	pub(super) fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> TextureState {
		let textures: Vec<_> = create_texture_iter(device, queue)
			.chain(create_animation_texture_iter(device, queue))
			.collect();

		let texture_views: Vec<wgpu::TextureView> = textures.iter()
			.map(|texture| create_texture_view(texture))
			.collect();

		TextureState {
			textures,
			texture_views,
		}
	}

	#[allow(unused)]
	pub(super) fn texture(&self, index: impl IntoTextureIndex) -> &wgpu::Texture {
		let index = index.into_texture_index();
		&self.textures[index]
	}

	#[allow(unused)]
	pub(super) fn texture_view(&self, index: impl IntoTextureIndex) -> &wgpu::TextureView {
		let index = index.into_texture_index();
		&self.texture_views[index]
	}

	pub(super) fn texture_view_iter(&self) -> impl Iterator<Item=&wgpu::TextureView> {
		self.texture_views.iter()
	}
}
