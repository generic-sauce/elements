mod world;
pub use world::*;

mod surface_vec;
pub use surface_vec::*;

mod texture_state2;
pub use texture_state2::*;

mod draw_triangles;
mod draw_tilemap;
mod draw_fluidmap;



use crate::prelude::*;
use draw_triangles::*;
use draw_tilemap::*;
use draw_fluidmap::*;

pub const SURFACE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;

pub struct DrawContext2 {
	window_size: Vec2u,
}

pub struct Graphics {
	#[allow(unused)] instance: wgpu::Instance,
	surface: wgpu::Surface,
	device: wgpu::Device,
	queue: wgpu::Queue,
	window_size: Vec2u,
	swap_chain: wgpu::SwapChain,
	triangles: DrawTriangles,
	tilemap: DrawTilemap,
	fluidmap: DrawFluidmap,
}

fn create_swap_chain(device: &wgpu::Device, surface: &wgpu::Surface, size: Vec2u) -> wgpu::SwapChain {
	let swap_chain_desc = wgpu::SwapChainDescriptor {
		usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
		format: SURFACE_FORMAT,
		width: size.x,
		height: size.y,
		present_mode: wgpu::PresentMode::Immediate,
	};

	device.create_swap_chain(surface, &swap_chain_desc)
}

impl Graphics {
	/* create instance
	 * create surface
	 * choose adapter (physical device)
	 * get device
	 */
	pub fn new(window: &win::Window) -> Graphics {
		let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

		let surface = unsafe { instance.create_surface(window) };
		let adapter = futures::executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
			power_preference: wgpu::PowerPreference::HighPerformance,
			compatible_surface: Some(&surface)
		}))
			.unwrap();

		let (device, queue) = futures::executor::block_on(adapter.request_device(
			&wgpu::DeviceDescriptor {
				features: wgpu::Features::empty(),
				limits: Default::default(),
				shader_validation: true
			},
			None
		))
			.unwrap();

		let window_size = window.inner_size();
		let window_size = Vec2u::new(window_size.width, window_size.height);

		let swap_chain = create_swap_chain(&device, &surface, window_size);

		let triangles = DrawTriangles::new(&device, &queue);
		let tilemap = DrawTilemap::new(&device, &queue);
		let fluidmap = DrawFluidmap::new(&device, &queue);

		Graphics {
			instance,
			surface,
			device,
			queue,
			window_size,
			swap_chain,
			triangles,
			tilemap,
			fluidmap,
		}
	}

	pub fn draw(&mut self, world: &GraphicsWorld) {
		let window_size = Vec2u::new(self.window_size.x as u32, self.window_size.y as u32);
		let context = DrawContext2 {
			window_size,
		};

		// self.draw_hud(...);
		self.draw_players(&context, &world);
		self.draw_cursors(&context, &world);
		// self.draw_cursors(&context, &world);
		// self.draw_tilemap(&world);
		// self.draw_fluids(...);
		// self.draw_background(...);
	}

	/* create and fill draw pass
	 * create and fill command buffer
	 * submit command buffer to queue
	 */
	pub fn flush(&mut self, world: &GraphicsWorld) {
		let swap_chain_texture = self.swap_chain
			.get_current_frame()
			.unwrap()
			.output;

		let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
			label: Some("command encoder")
		});

		let clear_color = wgpu::Color {
			r: 50.0 / 255.0,
			g: 120.0 / 255.0,
			b: 215.0 / 255.0,
			a: 1.0,
		};

		self.fluidmap.render(
			&self.device,
			&self.queue,
			&mut encoder,
			&swap_chain_texture,
			wgpu::LoadOp::Clear(clear_color),
			&world,
		);

		self.tilemap.render(
			&self.device,
			&self.queue,
			&mut encoder,
			&swap_chain_texture,
			wgpu::LoadOp::Load,
			world.tilemap_size,
			&world.tilemap_data,
		);

		self.triangles.flush(
			&self.device,
			&self.queue,
			&mut encoder,
			&swap_chain_texture,
			wgpu::LoadOp::Load,
		);

		self.queue.submit(Some(encoder.finish()));
	}

	pub fn resize(&mut self, size: Vec2u) {
		self.window_size = size;
		self.swap_chain = create_swap_chain(&self.device, &self.surface, size);
	}
}
