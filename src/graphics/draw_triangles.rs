use crate::prelude::*;

#[derive(Copy, Clone)]
struct Vertex {
	position: CanvasVec,
	uv: CanvasVec,
	color: wgpu::Color,
}

type Triangle = [Vertex; 3];

fn triangles_to_bytes(triangles: &[Triangle]) -> Vec<u8> {
	let floats_per_vertex = 7;
	let floats_per_triangle = 3 * floats_per_vertex;
	let floats_in_triangles = triangles.len() * floats_per_triangle;
	let bytes_in_triangles = floats_in_triangles * std::mem::size_of::<f32>();
	let mut bytes = Vec::<u8>::with_capacity(bytes_in_triangles);

	for triangle in triangles {
		for vertex in triangle {
			bytes.extend(vertex.position.x.to_le_bytes().iter());
			bytes.extend(vertex.position.y.to_le_bytes().iter());
			bytes.extend(vertex.uv.x.to_le_bytes().iter());
			bytes.extend(vertex.uv.y.to_le_bytes().iter());
			bytes.extend((vertex.color.r as f32).to_le_bytes().iter());
			bytes.extend((vertex.color.g as f32).to_le_bytes().iter());
			bytes.extend((vertex.color.b as f32).to_le_bytes().iter());
		}
	}

	bytes
}

pub struct DrawTriangles {
	pipeline: wgpu::RenderPipeline,
	// texture: wgpu::Texture,
	// texture_view: wgpu::TextureView,
	triangles: Vec<Triangle>,
	vertex_buffer: wgpu::Buffer,
	triangles_capacity: u64,
}

impl DrawTriangles {
	fn create_vertex_buffer(device: &wgpu::Device, triangles_capacity: u64) -> wgpu::Buffer {
		let triangle_size = std::mem::size_of::<Triangle>() as u64;
		let triangles_size = triangles_capacity * triangle_size;
		let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
			label: Some("vertex buffer"),
			size: triangles_size,
			usage: wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::VERTEX,
			mapped_at_creation: false
		});

		vertex_buffer
	}

	fn enlarge_vertex_buffer(&mut self, device: &wgpu::Device, min_triangles_capacity: u64) {
		while self.triangles_capacity < min_triangles_capacity {
			self.triangles_capacity = self.triangles_capacity * 2;
		}
		self.vertex_buffer = Self::create_vertex_buffer(device, self.triangles_capacity);
	}

	pub fn new(device: &wgpu::Device) -> DrawTriangles {
		let triangles_capacity = 128 as u64;
		let vertex_buffer = Self::create_vertex_buffer(device, triangles_capacity);

		let vertex_buffer_desc = wgpu::VertexBufferDescriptor {
			stride: 7 * std::mem::size_of::<f32>() as u64,
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
				wgpu::VertexAttributeDescriptor {
					offset: 4 * std::mem::size_of::<f32>() as u64,
					format: wgpu::VertexFormat::Float3,
					shader_location: 2
				},
			]
		};

		let vert = device.create_shader_module(wgpu::include_spirv!("../../res/shader/triangles.vert.spv"));
		let frag = device.create_shader_module(wgpu::include_spirv!("../../res/shader/triangles.frag.spv"));

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
			primitive_topology: wgpu::PrimitiveTopology::TriangleList,
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

		let triangles = Vec::with_capacity(triangles_capacity as usize);

		DrawTriangles {
			pipeline,
			triangles,
			vertex_buffer,
			triangles_capacity
		}
	}

	fn render(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, encoder: &mut wgpu::CommandEncoder, swap_chain_texture: &wgpu::SwapChainTexture, load: wgpu::LoadOp::<wgpu::Color>) {
		if self.triangles_capacity < self.triangles.len() as u64 {
			self.enlarge_vertex_buffer(device, self.triangles.len() as u64);
		}

		queue.write_buffer(&self.vertex_buffer, 0, &triangles_to_bytes(&self.triangles[..]));

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
		render_pass.draw(0 .. (3 * self.triangles.len() as u32), 0 .. 1);
	}

	pub fn clear(&mut self) {
		// keep capacity
		self.triangles.clear();
	}

	pub fn flush(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, encoder: &mut wgpu::CommandEncoder, swap_chain_texture: &wgpu::SwapChainTexture, load: wgpu::LoadOp::<wgpu::Color>) {
		self.render(device, queue, encoder, swap_chain_texture, load);
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

	#[allow(unused)]
	pub fn draw_sprite(&mut self, left_bot: CanvasVec, size: CanvasVec, color: Option<wgpu::Color>) {
		let color = if let Some(color) = color { color } else { wgpu::Color::WHITE };
		self.triangles.push([
			Vertex { position: left_bot, uv: CanvasVec::new(0.0, 0.0), color: color },
			Vertex { position: left_bot + (size.x, 0.0), uv: CanvasVec::new(1.0, 0.0), color: color },
			Vertex { position: left_bot + size, uv: CanvasVec::new(1.0, 1.0), color: color },
		]);

		self.triangles.push([
			Vertex { position: left_bot, uv: CanvasVec::new(0.0, 0.0), color: color },
			Vertex { position: left_bot + size, uv: CanvasVec::new(1.0, 1.0), color: color },
			Vertex { position: left_bot + (0.0, size.y), uv: CanvasVec::new(0.0, 1.0), color: color },
		]);
	}
}
