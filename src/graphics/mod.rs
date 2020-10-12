use crate::prelude::*;

const surface_format: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;

pub struct Graphics {
	instance: wgpu::Instance,
	surface: wgpu::Surface,
	device: wgpu::Device,
	queue: wgpu::Queue,
	window_size: Vec2u,
	swap_chain: wgpu::SwapChain,
	pipeline: wgpu::RenderPipeline,
}

impl Graphics {
	pub fn new(window: win::Window) -> (win::Window, Graphics) {
		let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

		let surface = unsafe { instance.create_surface(&window) };
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

		let vert = device.create_shader_module(wgpu::include_spirv!("../../res/shader/shader.vert.spv"));
		let frag = device.create_shader_module(wgpu::include_spirv!("../../res/shader/shader.frag.spv"));

		let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			label: Some("bind group layout"),
			entries: &[]
		});

		let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: Some("pipeline layout descriptor"),
			bind_group_layouts: &[
				// &bind_group_layout,
			],
			push_constant_ranges: &[]
		});

		let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: Some("render pipeline"),
			layout: Some(&pipeline_layout),
			vertex_stage: wgpu::ProgrammableStageDescriptor {
					module: &vert,
					entry_point: "main",
			},
			fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
					module: &frag,
					entry_point: "main",
			}),
			rasterization_state: Some(wgpu::RasterizationStateDescriptor {
					front_face: wgpu::FrontFace::Ccw,
					cull_mode: wgpu::CullMode::Back,
					..Default::default()
			}),
			primitive_topology: wgpu::PrimitiveTopology::TriangleList,
			color_states: &[surface_format.into()],
			depth_stencil_state: None,
			vertex_state: wgpu::VertexStateDescriptor {
				index_format: Default::default(),
				vertex_buffers: &[],
			},
			sample_count: 1,
			sample_mask: !0,
			alpha_to_coverage_enabled: false,
		});

		let mut swap_chain_desc = wgpu::SwapChainDescriptor {
			usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
			format: surface_format,
			width: window_size.x,
			height: window_size.y,
			present_mode: wgpu::PresentMode::Fifo,
		};

		let mut swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

		(window, Graphics {
			instance,
			surface,
			device,
			queue,
			window_size,
			swap_chain,
			pipeline,
		})
	}

	pub fn render(&mut self) {
		let frame = self.swap_chain
			.get_current_frame()
			.unwrap()
			.output;

		let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
			label: Some("command encoder")
		});

		{
			let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				color_attachments: &[
					wgpu::RenderPassColorAttachmentDescriptor {
						attachment: &frame.view,
						resolve_target: None,
						ops: wgpu::Operations {
							load: wgpu::LoadOp::Clear(wgpu::Color {
								r: 0.03,
								g: 0.025,
								b: 0.025,
								a: 1.0,
							}),
							store: true
						}
					},
				],
				depth_stencil_attachment: None
			});

			render_pass.set_pipeline(&self.pipeline);
			// render_pass.set_bind_group(
			// 	0,
			// 	&bind_group,
			// 	&[]
			// );
			// render_pass.set_vertex_buffer(0, vert_buffer.slice(..));
			render_pass.draw(0..3, 0..1);
		}

		self.queue.submit(Some(encoder.finish()));
	}

	pub fn resize(&mut self, size: Vec2u) {
		let mut swap_chain_desc = wgpu::SwapChainDescriptor {
			usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
			format: surface_format,
			width: size.x,
			height: size.y,
			present_mode: wgpu::PresentMode::Fifo,
		};

		self.window_size = size;
		self.swap_chain = self.device.create_swap_chain(&self.surface, &swap_chain_desc);
	}
}
