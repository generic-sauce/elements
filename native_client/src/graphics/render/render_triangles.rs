use crate::graphics::*;
use super::*;

pub(in crate::graphics) struct RenderTriangles {
	pipeline: wgpu::RenderPipeline,
	capacity: u64,
	vertex_buffer: wgpu::Buffer,
	#[allow(dead_code)] texture_state: TextureState,
	#[allow(dead_code)] sampler: wgpu::Sampler,
	#[allow(dead_code)] bind_group_layout: wgpu::BindGroupLayout,
	bind_groups: Vec<wgpu::BindGroup>,
}

impl RenderTriangles {
	pub(in crate::graphics) fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> RenderTriangles {
		let texture_state = TextureState::new(device, queue);

		let capacity = 128 as u64; // smallest power of 2 to render a texture twice
		let vertex_buffer = Self::create_vertex_buffer(device, capacity);

		let vertex_buffer_desc = wgpu::VertexBufferDescriptor {
			stride: 8 * std::mem::size_of::<f32>() as u64,
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
					format: wgpu::VertexFormat::Float4,
					shader_location: 2
				},
			]
		};

		let vert = load_shader_from_file(device, res("shader/triangles.vert.spv"));
		let frag = load_shader_from_file(device, res("shader/triangles.frag.spv"));

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
			depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
				format: wgpu::TextureFormat::Depth32Float,
				depth_write_enabled: true,
				depth_compare: wgpu::CompareFunction::Less,
				stencil: Default::default(),
			}),
			vertex_state: wgpu::VertexStateDescriptor {
				index_format: Default::default(),
				vertex_buffers: &[vertex_buffer_desc],
			},
			sample_count: 1,
			sample_mask: !0,
			alpha_to_coverage_enabled: false,
		});

		// TODO linear or nearest?
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

		RenderTriangles {
			pipeline,
			capacity,
			vertex_buffer,
			texture_state,
			sampler,
			bind_group_layout,
			bind_groups,
		}
	}

	pub(in crate::graphics) fn set_vertices(
		&mut self,
		context: &mut GraphicsContext,
		draw: &RenderDraw
	) {
		self.enlarge_vertex_buffer(context.device, draw.triangle_data.len() as u64);
		context.queue.write_buffer(&self.vertex_buffer, 0, &draw.triangle_data[..]);
	}

	pub(in crate::graphics) fn render(
		&mut self,
		context: &mut GraphicsContext,
		texture_index: TextureIndex,
		from: VertexIndex,
		to: VertexIndex,
	) {
		let color_load_op = context.color_load_op();
		let depth_load_op = context.depth_load_op();
		let mut render_pass = context.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
			color_attachments: &[
				wgpu::RenderPassColorAttachmentDescriptor {
					attachment: &context.swap_chain_texture.view,
					resolve_target: None,
					ops: wgpu::Operations {
						load: color_load_op,
						store: true
					}
				},
			],
			depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
				attachment: context.depth_texture_view,
				depth_ops: Some(wgpu::Operations {
					load: depth_load_op,
					store: true,
				}),
				stencil_ops: None,
			}),
		});

		render_pass.set_pipeline(&self.pipeline);

		render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(from as u64 .. to as u64));
		render_pass.set_bind_group(0, &self.bind_groups[texture_index], &[]);
		render_pass.draw(0 .. (to - from) as u32, 0 .. 1);
	}

	fn create_vertex_buffer(device: &wgpu::Device, size: u64) -> wgpu::Buffer {
		device.create_buffer(&wgpu::BufferDescriptor {
			label: Some("vertex buffer"),
			size,
			usage: wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::VERTEX,
			mapped_at_creation: false
		})
	}

	fn enlarge_vertex_buffer(&mut self, device: &wgpu::Device, min_capacity: u64) {
		while self.capacity < min_capacity {
			self.capacity *= 2;
		}
		self.vertex_buffer = Self::create_vertex_buffer(device, self.capacity);
	}
}
