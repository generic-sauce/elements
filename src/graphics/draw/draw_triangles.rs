use crate::graphics::*;
use super::*;

fn triangles_to_bytes(triangles: &[Triangle]) -> Vec<u8> {
	let floats_per_vertex = 7;
	let floats_per_triangle = 3 * floats_per_vertex;
	let floats_in_triangles = triangles.len() * floats_per_triangle;
	let bytes_in_triangles = floats_in_triangles * std::mem::size_of::<f32>();
	let mut bytes = Vec::<u8>::with_capacity(bytes_in_triangles);

	for triangle in triangles {
		for vertex in triangle {
			let l = [
				vertex.position.x.to_le_bytes(),
				vertex.position.y.to_le_bytes(),
				vertex.uv.x.to_le_bytes(),
				vertex.uv.y.to_le_bytes(),
				vertex.color.r.to_le_bytes(),
				vertex.color.g.to_le_bytes(),
				vertex.color.b.to_le_bytes(),
			];
			bytes.extend(l.iter().flat_map(|x| x.iter()));
		}
	}

	bytes
}

pub(in crate::graphics) struct DrawTriangles {
	pipeline: wgpu::RenderPipeline,
	triangles_capacity: u64,
	vertex_buffer: wgpu::Buffer,
	#[allow(dead_code)] texture_state: TextureState,
	#[allow(dead_code)] sampler: wgpu::Sampler,
	#[allow(dead_code)] bind_group_layout: wgpu::BindGroupLayout,
	bind_groups: Vec<wgpu::BindGroup>,
}

impl DrawTriangles {
	fn create_vertex_buffer(device: &wgpu::Device, triangles_capacity: u64) -> wgpu::Buffer {
		let triangle_size = std::mem::size_of::<Triangle>() as u64;
		let triangles_size = triangles_capacity * triangle_size;
		device.create_buffer(&wgpu::BufferDescriptor {
			label: Some("vertex buffer"),
			size: triangles_size,
			usage: wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::VERTEX,
			mapped_at_creation: false
		})
	}

	fn enlarge_vertex_buffer(&mut self, device: &wgpu::Device, min_triangles_capacity: u64) {
		while self.triangles_capacity < min_triangles_capacity {
			self.triangles_capacity *= 2;
		}
		self.vertex_buffer = Self::create_vertex_buffer(device, self.triangles_capacity);
	}

	pub(in crate::graphics) fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> DrawTriangles {
		let texture_state = TextureState::new(device, queue);

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

		let vert = device.create_shader_module(wgpu::include_spirv!("../../../res/shader/triangles.vert.spv"));
		let frag = device.create_shader_module(wgpu::include_spirv!("../../../res/shader/triangles.frag.spv"));

		let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			label: Some("bind group layout"),
			entries: &[
				wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: wgpu::ShaderStage::FRAGMENT,
					count: None,
					ty: wgpu::BindingType::SampledTexture {
						dimension: wgpu::TextureViewDimension::D2,
						component_type: wgpu::TextureComponentType::Float,
						multisampled: false
					},
				},
				wgpu::BindGroupLayoutEntry {
					binding: 1,
					visibility: wgpu::ShaderStage::FRAGMENT,
					count: None,
					ty: wgpu::BindingType::Sampler {
						comparison: false
					},
				},
			]
		});

		let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: Some("pipeline layout descriptor"),
			bind_group_layouts: &[
				&bind_group_layout,
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

		let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
			label: Some("fluidmap sampler"),
			..Default::default()
		});

		let bind_groups = texture_state.texture_view_iter()
			.map(|texture_view|
				device.create_bind_group(&wgpu::BindGroupDescriptor {
					label: Some("bind group"),
					layout: &bind_group_layout,
					entries: &[
						wgpu::BindGroupEntry {
							binding: 0,
							resource: wgpu::BindingResource::TextureView(&texture_view),
						},
						wgpu::BindGroupEntry {
							binding: 1,
							resource: wgpu::BindingResource::Sampler(&sampler),
						},
					]
				})
			)
			.collect();

		DrawTriangles {
			pipeline,
			triangles_capacity,
			vertex_buffer,
			texture_state,
			sampler,
			bind_group_layout,
			bind_groups,
		}
	}

	pub(in crate::graphics) fn render(
		&mut self,
		context: &mut GraphicsContext,
		load: wgpu::LoadOp::<wgpu::Color>,
		draw: &Draw,
	) {
		let max_triangles: usize = draw.triangles.iter()
			.map(|x| x.len())
			.sum();
		self.enlarge_vertex_buffer(context.device, max_triangles as u64);

		let mut render_pass = context.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
			color_attachments: &[
				wgpu::RenderPassColorAttachmentDescriptor {
					attachment: &context.swap_chain_texture.view,
					resolve_target: None,
					ops: wgpu::Operations {
						load,
						store: true
					}
				},
			],
			depth_stencil_attachment: None
		});

		render_pass.set_pipeline(&self.pipeline);

		// copy all triangles
		let mut all_bytes = Vec::new();
		let mut slice_end = 0;
		let mut slice_ends = Vec::new();
		for triangles in draw.triangles.iter() {
			let bytes = triangles_to_bytes(&triangles[..]);
			slice_end += bytes.len();
			slice_ends.push(slice_end as u64);
			all_bytes.extend(&bytes);
		}
		context.queue.write_buffer(&self.vertex_buffer, 0, &all_bytes[..]);

		let mut slice_begin = 0;
		for (i, triangles) in draw.triangles.iter().enumerate() {
			if !triangles.is_empty() {
				let slice_end = slice_ends[i];
				render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(slice_begin .. slice_end));
				slice_begin = slice_end;
				render_pass.set_bind_group(0, &self.bind_groups[i], &[]);
				render_pass.draw(0 .. (3 * triangles.len() as u32), 0 .. 1);
			}
		}
	}
}
