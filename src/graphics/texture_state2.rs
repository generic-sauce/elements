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

				let texture = device.create_texture(&wgpu::TextureDescriptor {
					label: None,
					size: wgpu::Extent3d {
						width: size.0,
						height: size.1,
						depth: 1
					},
					mip_level_count: 1,
					sample_count: 1,
					dimension: wgpu::TextureDimension::D2,
					format: wgpu::TextureFormat::Rgba8Unorm,
					usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST
				});

				queue.write_texture(
					wgpu::TextureCopyView {
						texture: &texture,
						mip_level: 0,
						origin: wgpu::Origin3d::ZERO,
					},
					&image.as_raw()[..],
					wgpu::TextureDataLayout {
						offset: 0,
						bytes_per_row: 4 * size.0,
						rows_per_image: size.1,
					},
					wgpu::Extent3d {
						width: size.0,
						height: size.1,
						depth: 1,
					}
				);

				texture
			})
			.collect();

		let texture_views: Vec<wgpu::TextureView> = textures.iter()
			.map(|texture| texture.create_view(&wgpu::TextureViewDescriptor::default()))
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
}
