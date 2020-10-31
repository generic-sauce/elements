mod texture;
pub use texture::*;

mod animation;
pub use animation::*;

mod misc;
pub use misc::*;

pub trait IntoTextureIndex {
	fn into_texture_index(self) -> usize;
}

#[derive(PartialEq, Eq)]
pub enum Flip2 {
	Normal,
	Horizontal,
}

pub struct TextureState2 {
	textures: Vec<wgpu::Texture>,
	texture_views: Vec<wgpu::TextureView>,
}

impl TextureState2 {
	pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> TextureState2 {
		let textures: Vec<_> = create_texture_iter(device, queue)
			.chain(create_animation_texture_iter(device, queue))
			.collect();

		let texture_views: Vec<wgpu::TextureView> = textures.iter()
			.map(|texture| create_texture_view(texture))
			.collect();

		TextureState2 {
			textures,
			texture_views,
		}
	}

	pub fn texture(&self, index: impl IntoTextureIndex) -> &wgpu::Texture {
		let index = index.into_texture_index();
		&self.textures[index]
	}

	pub fn texture_view(&self, index: impl IntoTextureIndex) -> &wgpu::TextureView {
		let index = index.into_texture_index();
		&self.texture_views[index]
	}

	pub fn texture_count(&self) -> usize {
		self.textures.len()
	}

	pub fn texture_view_iter(&self) -> impl Iterator<Item=&wgpu::TextureView> {
		self.texture_views.iter()
	}
}
