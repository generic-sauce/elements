use crate::prelude::*;

// TODO this thing is not used currently!

macro_rules! setup {
	($($id:ident : $resource:expr),*) => {

		#[derive(Copy, Clone, Debug)]
		#[repr(usize)]
		#[allow(unused)]
		pub enum TextureId2 {
			$($id),*
		}

		impl TextureId2 {
			#[allow(unused)]
			pub fn iter() -> impl Iterator<Item=TextureId2> {
				[$(TextureId2::$id),*].iter().cloned()
			}

			#[allow(unused)]
			pub fn filepath(self) -> String {
				match self {
					$(
						TextureId2::$id => res($resource),
					)*
				}
			}
		}
	}
}

setup!(
	Unknown: "images/checkerboard.png",
	White: "images/white.png",
	BluePlayerIdle1: "images/player_blue/player_idle/player_idle1.png"
);

pub fn create_texture(device: &wgpu::Device, size: Vec2u) -> wgpu::Texture {
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

pub fn write_texture(queue: &wgpu::Queue, texture: &wgpu::Texture, size: Vec2u, image: &[u8]) {
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

pub fn create_texture_view(texture: &wgpu::Texture) -> wgpu::TextureView {
	texture.create_view(&wgpu::TextureViewDescriptor::default())
}

pub struct TextureState2 {
	textures: Vec<wgpu::Texture>,
	texture_views: Vec<wgpu::TextureView>,
}

impl TextureState2 {
	pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> TextureState2 {
		let textures: Vec<wgpu::Texture> = TextureId2::iter()
			.map(|id| TextureId2::filepath(id))
			.map(|filepath| image::open(filepath).unwrap().flipv().into_rgba())
			.map(|image| {
				let size = image.dimensions();
				let size = Vec2u::new(size.0, size.1);
				let texture = create_texture(device, size);
				write_texture(queue, &texture, size, &image.as_raw()[..]);
				texture
			})
			.collect();

		let texture_views: Vec<wgpu::TextureView> = textures.iter()
			.map(|texture| create_texture_view(texture))
			.collect();

		TextureState2 {
			textures,
			texture_views
		}
	}

	#[allow(unused)]
	pub fn texture(&self, id: TextureId2) -> &wgpu::Texture {
		&self.textures[id as usize]
	}

	#[allow(unused)]
	pub fn texture_view(&self, id: TextureId2) -> &wgpu::TextureView {
		&self.texture_views[id as usize]
	}

	#[allow(unused)]
	pub fn texture_count(&self) -> usize {
		self.textures.len()
	}
}

#[derive(PartialEq, Eq)]
pub enum Flip2 {
	Normal,
	Horizontal,
}
