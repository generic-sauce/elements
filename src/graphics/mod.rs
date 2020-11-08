use crate::prelude::*;

mod draw;
use draw::*;

mod context;
use context::*;

mod misc;
use misc::*;

const SURFACE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;

impl Color {
	fn to_wgpu(self) -> wgpu::Color {
		wgpu::Color {
			r: self.r as f64,
			g: self.g as f64,
			b: self.b as f64,
			a: self.a as f64,
		}
	}
}

pub struct Graphics {
	#[allow(unused)] instance: wgpu::Instance,
	surface: wgpu::Surface,
	device: wgpu::Device,
	queue: wgpu::Queue,
	window_size: WindowVec,
	swap_chain: wgpu::SwapChain,
	depth_texture: wgpu::Texture,
	depth_texture_view: wgpu::TextureView,
	triangles: DrawTriangles,
	tilemap: DrawTilemap,
	fluidmap: DrawFluidmap,
	text: DrawText,
}

fn create_swap_chain(device: &wgpu::Device, surface: &wgpu::Surface, window_size: WindowVec) -> wgpu::SwapChain {
	let swap_chain_desc = wgpu::SwapChainDescriptor {
		usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
		format: SURFACE_FORMAT,
		width: window_size.x,
		height: window_size.y,
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
		let window_size = WindowVec::new(window_size.width, window_size.height);

		let swap_chain = create_swap_chain(&device, &surface, window_size);

		let depth_texture = create_depth_texture(&device, window_size);
		let depth_texture_view = create_texture_view(&depth_texture);

		let triangles = DrawTriangles::new(&device, &queue);
		let tilemap = DrawTilemap::new(&device);
		let fluidmap = DrawFluidmap::new(&device);
		let text = DrawText::new(&device);

		Graphics {
			instance,
			surface,
			device,
			queue,
			window_size,
			swap_chain,
			depth_texture,
			depth_texture_view,
			triangles,
			tilemap,
			fluidmap,
			text,
		}
	}

	/* create and fill draw pass
	 * create and fill command buffer
	 * submit command buffer to queue
	 */
	pub fn render(&mut self, draw: &Draw) {
		let swap_chain_texture = match self.swap_chain.get_current_frame() {
			Ok(frame) => frame.output,
			Err(err) => {
				println!("swap chain error: {}. recreate and try again...", match err {
					wgpu::SwapChainError::Timeout => "timeout",
					wgpu::SwapChainError::Outdated => "outdated",
					wgpu::SwapChainError::Lost => "lost",
					wgpu::SwapChainError::OutOfMemory => "out of memory",
				});

				self.swap_chain = create_swap_chain(&self.device, &self.surface, self.window_size);
				self.swap_chain.get_current_frame().unwrap().output
			},
		};

		let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
			label: Some("command encoder")
		});

		let mut graphics_context = GraphicsContext::new(
			&self.device,
			&self.queue,
			&swap_chain_texture,
			&mut encoder,
			self.window_size,
			&self.depth_texture_view,
			draw.clear_color.unwrap_or(Color::BLACK).to_wgpu()
		);

		if let Some(world) = &draw.world {
			self.fluidmap.render(
				&mut graphics_context,
				&world,
			);

			self.tilemap.render(
				&mut graphics_context,
				&world,
			);
		}

		self.triangles.render(
			&mut graphics_context,
			draw,
		);

		self.text.render(
			&mut graphics_context,
			draw,
		);

		self.queue.submit(Some(encoder.finish()));
	}

	pub fn resize(&mut self, size: WindowVec) {
		self.window_size = size;
		self.swap_chain = create_swap_chain(&self.device, &self.surface, size);

		self.depth_texture = create_depth_texture(&self.device, self.window_size);
		self.depth_texture_view = create_texture_view(&self.depth_texture);
	}
}
