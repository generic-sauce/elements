use crate::prelude::*;

#[derive(Copy, Clone)]
struct Vertex {
	position: CanvasVec,
	uv: CanvasVec,
}

fn vertex_to_bytes_len() -> u64 {
	2 * 2 * std::mem::size_of::<f32>() as u64
}

fn vertices_to_bytes(vertices: &[Vertex]) -> Vec<u8> {
	let vertices_size = vertices.len() * vertex_to_bytes_len() as usize;
	let mut bytes = Vec::<u8>::with_capacity(vertices_size);

	for vertex in vertices {
		bytes.extend(vertex.position.x.to_le_bytes().iter());
		bytes.extend(vertex.position.y.to_le_bytes().iter());
		bytes.extend(vertex.uv.x.to_le_bytes().iter());
		bytes.extend(vertex.uv.y.to_le_bytes().iter());
	}

	bytes
}

pub struct DrawTilemap {
	pipeline: wgpu::RenderPipeline,
	vertex_buffer: wgpu::Buffer,
}

impl DrawTilemap {
	fn create_vertex_buffer(device: &wgpu::Device, vertices_capacity: u64) -> wgpu::Buffer {
		let vertices_size = vertices_capacity * vertex_to_bytes_len();
		let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
			label: Some("vertex buffer"),
			size: vertices_size,
			usage: wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::VERTEX,
			mapped_at_creation: false
		});

		vertex_buffer
	}

	pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> DrawTilemap {
		let vertex_buffer = Self::create_vertex_buffer(device, 4);
		queue.write_buffer(&vertex_buffer, 0, &vertices_to_bytes(&vec!(
			Vertex { position: CanvasVec::new(0.0, 0.0), uv: CanvasVec::new(0.0, 0.0) },
			Vertex { position: CanvasVec::new(1.0, 0.0), uv: CanvasVec::new(1.0, 0.0) },
			Vertex { position: CanvasVec::new(0.0, 1.0), uv: CanvasVec::new(0.0, 1.0) },
			Vertex { position: CanvasVec::new(1.0, 1.0), uv: CanvasVec::new(1.0, 1.0) },
		))[..]);

		let vertex_buffer_desc = wgpu::VertexBufferDescriptor {
			stride: vertex_to_bytes_len(),
			step_mode: wgpu::InputStepMode::Vertex,
			attributes: &[
				wgpu::VertexAttributeDescriptor {
					offset: 0,
					format: wgpu::VertexFormat::Float2,
					shader_location: 0
				},
				wgpu::VertexAttributeDescriptor {
					offset: 2 * std::mem::size_of::<f32>() as u64,
					format: wgpu::VertexFormat::Float2,
					shader_location: 1
				},
			]
		};

		let vert = device.create_shader_module(wgpu::include_spirv!("../../res/shader/tilemap.vert.spv"));
		let frag = device.create_shader_module(wgpu::include_spirv!("../../res/shader/tilemap.frag.spv"));

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
			label: Some("Render pipeline"),
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
					cull_mode: wgpu::CullMode::None,
					..Default::default()
			}),
			primitive_topology: wgpu::PrimitiveTopology::TriangleStrip,
			color_states: &[SURFACE_FORMAT.into()],
			depth_stencil_state: None,
			vertex_state: wgpu::VertexStateDescriptor {
				index_format: Default::default(),
				vertex_buffers: &[vertex_buffer_desc],
			},
			sample_count: 1,
			sample_mask: !0,
			alpha_to_coverage_enabled: false,
		});

		DrawTilemap {
			pipeline,
			vertex_buffer,
		}
	}

	pub fn render(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, encoder: &mut wgpu::CommandEncoder, swap_chain_texture: &wgpu::SwapChainTexture, load: wgpu::LoadOp::<wgpu::Color>) {
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
		// draw_pass.set_bind_group(
		// 	0,
		// 	&bind_group,
		// 	&[]
		// );
		render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
		render_pass.draw(0 .. 4, 0 .. 1);
	}
}
