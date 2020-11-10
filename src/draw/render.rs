use crate::prelude::*;

fn triangles_to_bytes(window_size: SubPixelVec, triangles: &[Triangle], max_depth: DepthIndex) -> Vec<u8> {
	let floats_per_vertex = 7;
	let floats_per_triangle = 3 * floats_per_vertex;
	let floats_in_triangles = triangles.len() * floats_per_triangle;
	let bytes_in_triangles = floats_in_triangles * std::mem::size_of::<f32>();
	let mut bytes = Vec::<u8>::with_capacity(bytes_in_triangles);

	for triangle in triangles {
		let depth = (max_depth - triangle.depth_index + 0.01) / (max_depth + 0.02);
		for vertex in triangle.vertices.iter().rev() {
			let position = vertex.position.to_surface(window_size);
			let l = [
				position.x.to_le_bytes(),
				position.y.to_le_bytes(),
				depth.to_le_bytes(),
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

pub struct RenderDraw {
	pub clear_color: Color,
	// pub texts: Vec<Text>,
	pub world: Option<GraphicsWorld>,
	pub vertex_data: Vec<u8>, // vertices for all textures in bytes
	pub vertex_counts: Vec<u32>, // number of vertices per texture
}

impl RenderDraw {
	pub fn new(draw: Draw, window_size: SubPixelVec) -> RenderDraw {
		let Draw { clear_color, depth_index, texture_triangles, texts: _, world } = draw;

		let clear_color = clear_color.unwrap_or(Color::BLACK);
		let mut vertex_counts = Vec::new();
		let mut vertex_data = Vec::new();

		for triangles in texture_triangles {
			let count = 3 * triangles.len() as u32;
			let bytes = triangles_to_bytes(window_size, &triangles[..], depth_index);

			vertex_counts.push(count);
			vertex_data.extend(bytes);
		}

		RenderDraw {
			clear_color,
			// texts,
			world, // TODO bring to new format
			vertex_data,
			vertex_counts,
		}
	}
}
