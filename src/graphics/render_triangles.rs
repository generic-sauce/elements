use crate::prelude::*;

#[derive(Copy, Clone)]
pub struct Vertex {
	position: CanvasVec,
	uv: CanvasVec,
	color: wgpu::Color,
}

pub struct RenderTriangles {
	pipeline: wgpu::RenderPipeline,
	// texture: wgpu::Texture,
	// texture_view: wgpu::TextureView,
	buffer: Vec::<Vertex>,
}

impl RenderTriangles {
	pub fn new(device: &wgpu::Device) -> RenderTriangles {
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
			color_states: &[SURFACE_FORMAT.into()],
			depth_stencil_state: None,
			vertex_state: wgpu::VertexStateDescriptor {
				index_format: Default::default(),
				vertex_buffers: &[],
			},
			sample_count: 1,
			sample_mask: !0,
			alpha_to_coverage_enabled: false,
		});

		let buffer = Vec::with_capacity(1024);

		RenderTriangles {
			pipeline,
			buffer,
		}
	}

	pub fn render(&self, encoder: &mut wgpu::CommandEncoder, swap_chain_texture: &wgpu::SwapChainTexture, load: wgpu::LoadOp::<wgpu::Color>) {
		let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
			color_attachments: &[
				wgpu::RenderPassColorAttachmentDescriptor {
					attachment: &swap_chain_texture.view,
					resolve_target: None,
					ops: wgpu::Operations {
						load: load,
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
		render_pass.draw(0 .. self.buffer.len() as u32, 0 .. 1);
	}

	pub fn clear(&mut self) {
		// keep capacity
		self.buffer.clear();
	}

	pub fn flush(&mut self, encoder: &mut wgpu::CommandEncoder, swap_chain_texture: &wgpu::SwapChainTexture, load: wgpu::LoadOp::<wgpu::Color>) {
		self.render(encoder, swap_chain_texture, load);
		self.clear();
	}

	// struct LeftBot<T: IntoCanvasVec> (T)
	// struct Center<T: IntoCanvasVec> (T)
	// trait Origin {
	// 	pub fn left_bot(self) -> impl IntoCanvasVec;
	// }
	// impl Origin for LeftBot<T> {
	// 	pub fn left_bot(self) -> impl IntoCanvasVec {
	// 		self.0
	// 	}
	// }
  //
	// let v = GameVec::new(0, 0);
	// draw_rectangle(Origin::LeftBot(v));

	pub fn draw_rectangle(&mut self, left_bot: CanvasVec, size: CanvasVec, colors: Option<[Color; 4]>) {
		self.buffer.push(Vertex { position: left_bot, uv: CanvasVec::new(0.0, 0.0), color: wgpu::Color::WHITE });
		self.buffer.push(Vertex { position: left_bot + (size.x, 0.0), uv: CanvasVec::new(1.0, 0.0), color: wgpu::Color::WHITE });
		self.buffer.push(Vertex { position: left_bot + size, uv: CanvasVec::new(1.0, 1.0), color: wgpu::Color::WHITE });

		self.buffer.push(Vertex { position: left_bot, uv: CanvasVec::new(0.0, 0.0), color: wgpu::Color::WHITE });
		self.buffer.push(Vertex { position: left_bot + size, uv: CanvasVec::new(1.0, 1.0), color: wgpu::Color::WHITE });
		self.buffer.push(Vertex { position: left_bot + (0.0, size.y), uv: CanvasVec::new(0.0, 1.0), color: wgpu::Color::WHITE });
	}
}
