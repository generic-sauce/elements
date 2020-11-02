use super::*;

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
