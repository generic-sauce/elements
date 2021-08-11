use super::*;

pub struct RenderDraw {
	pub camera: Camera,
	pub clear_color: Color,
	pub commands: Vec<DrawCommand>,
	pub tilemap: Option<DrawTilemap>,
	pub fluidmap: Option<DrawFluidmap>,
	pub texts: Vec<Text>,
	pub triangle_data: Vec<u8>, // vertices for all textures in bytes
	pub triangle_commands: Vec<TriangleDrawCommand>,
}

impl RenderDraw {
	pub fn new(draw: Draw, window_size: SubPixelVec) -> RenderDraw {
		let Draw { camera, clear_color, commands, tilemap, fluidmap, texts, triangles, triangle_commands } = draw;

		let camera = camera.unwrap_or(Camera { left_bot: ViewVec::new(0.0, 0.0), zoom: 1.0 });
		let clear_color = clear_color.unwrap_or(Color::BLACK);

		let triangle_data = vertices_to_bytes(window_size, &triangles[..]);

		RenderDraw {
			camera,
			clear_color,
			commands,
			tilemap,
			fluidmap,
			texts,
			triangle_data,
			triangle_commands,
		}
	}
}

fn vertices_to_bytes(window_size: SubPixelVec, vertices: &[Vertex]) -> Vec<u8> {
	let mut bytes = Vec::with_capacity(vertices.len() * std::mem::size_of::<Vertex>());

	for vertex in vertices {
		let position = vertex.position.to_surface(window_size);
		let a = [
			position.x.to_le_bytes(),
			position.y.to_le_bytes(),
			vertex.uv.x.to_le_bytes(),
			vertex.uv.y.to_le_bytes(),
			vertex.color.r.to_le_bytes(),
			vertex.color.g.to_le_bytes(),
			vertex.color.b.to_le_bytes(),
			vertex.color.a.to_le_bytes(),
		];

		bytes.extend(a.iter().flat_map(|b| b.iter()));
	}

	bytes
}
