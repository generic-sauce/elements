use crate::graphics::*;

pub(super) struct TextureState {
	textures: Vec<wgpu::Texture>,
	texture_views: Vec<wgpu::TextureView>,
}

impl TextureState {
	pub(super) fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> TextureState {
		let textures: Vec<_> = texture_filenames_iter()
			.map(|filepath| image::open(&filepath).unwrap().flipv().into_rgba())
			.map(move |image| {
				let size: PixelVec = image.dimensions().into();
				let texture = create_texture(device, size);
				write_texture(queue, &texture, size, &image.as_raw()[..]);
				texture
			})
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
