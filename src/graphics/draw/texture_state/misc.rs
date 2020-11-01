use crate::prelude::*;

pub(super) fn create_texture(device: &wgpu::Device, size: Vec2u) -> wgpu::Texture {
	device.create_texture(&wgpu::TextureDescriptor {
		label: None,
		size: wgpu::Extent3d {
			width: size.x,
			height: size.y,
			depth: 1
		},
		mip_level_count: 1,
		sample_count: 1,
		dimension: wgpu::TextureDimension::D2,
		format: wgpu::TextureFormat::Rgba8Unorm,
		usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST
	})
}

pub(super) fn write_texture(queue: &wgpu::Queue, texture: &wgpu::Texture, size: Vec2u, image: &[u8]) {
	queue.write_texture(
		wgpu::TextureCopyView {
			texture: &texture,
			mip_level: 0,
			origin: wgpu::Origin3d::ZERO,
		},
		image,
		wgpu::TextureDataLayout {
			offset: 0,
			bytes_per_row: 4 * size.x,
			rows_per_image: size.y,
		},
		wgpu::Extent3d {
			width: size.x,
			height: size.y,
			depth: 1,
		}
	);
}

pub(super) fn create_texture_view(texture: &wgpu::Texture) -> wgpu::TextureView {
	texture.create_view(&wgpu::TextureViewDescriptor::default())
}
