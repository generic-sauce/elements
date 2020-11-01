use crate::prelude::*;

mod texture_state;
pub use texture_state::*;

const UNUSED_TILEVEC: TileVec = TileVec::new(0, 0);

mod draw_triangles;
pub(in crate::graphics) use draw_triangles::*;

mod draw_tilemap;
pub(in crate::graphics) use draw_tilemap::*;

mod draw_fluidmap;
pub(in crate::graphics) use draw_fluidmap::*;

mod world;
pub(in crate::graphics) use world::*;

#[derive(Copy, Clone)]
struct Vertex {
	pub position: CanvasVec,
	pub uv: Vec2f,
	pub color: wgpu::Color,
}

type Triangle = [Vertex; 3];
type Triangles = Vec<Triangle>;
type TextureTriangles = Vec<Triangles>;

pub struct Draw {
	triangles: TextureTriangles,
	pub(in crate::graphics) world: Option<GraphicsWorld>,
	pub(in crate::graphics) elapsed_time: Duration,
}

impl Draw {
	pub fn new(elapsed_time: Duration) -> Draw {
		let mut triangles = TextureTriangles::new();
		triangles.resize_with(TextureState::texture_count(), Default::default);
		Draw {
			triangles,
			world: None,
			elapsed_time,
		}
	}

	#[allow(unused)]
	pub fn texture(
		&mut self,
		left_bot: impl IntoCanvasVec,
		right_top: impl IntoCanvasVec,
		texture_index: impl IntoTextureIndex,
		flip: Flip,
		color: Option<wgpu::Color>
	) {
		let texture_index = texture_index.into_texture_index();
		let triangles = &mut self.triangles[texture_index];
		let left_bot = left_bot.to_canvas(UNUSED_TILEVEC);
		let right_top = right_top.to_canvas(UNUSED_TILEVEC);
		let color = color.unwrap_or(wgpu::Color::WHITE);
		let (left_uv, right_uv) = match flip {
			Flip::Normal => (0.0, 1.0),
			Flip::Horizontal => (1.0, 0.0),
		};

		triangles.push([
			Vertex { position: left_bot, uv: Vec2f::new(left_uv, 0.0), color: color },
			Vertex { position: v(right_top.x, left_bot.y), uv: Vec2f::new(right_uv, 0.0), color: color },
			Vertex { position: right_top, uv: Vec2f::new(right_uv, 1.0), color: color },
		]);

		triangles.push([
			Vertex { position: left_bot, uv: Vec2f::new(left_uv, 0.0), color: color },
			Vertex { position: right_top, uv: Vec2f::new(right_uv, 1.0), color: color },
			Vertex { position: v(left_bot.x, right_top.y), uv: Vec2f::new(left_uv, 1.0), color: color },
		]);
	}

	#[allow(unused)]
	pub fn rectangle(
		&mut self,
		left_bot: impl IntoCanvasVec,
		right_top: impl IntoCanvasVec,
		color: wgpu::Color
	) {
		let triangles = &mut self.triangles[TextureId::White as usize];
		let left_bot = left_bot.to_canvas(UNUSED_TILEVEC);
		let right_top = right_top.to_canvas(UNUSED_TILEVEC);

		triangles.push([
			Vertex { position: left_bot, uv: Vec2f::new(0.0, 0.0), color: color },
			Vertex { position: v(right_top.x, left_bot.y), uv: Vec2f::new(1.0, 0.0), color: color },
			Vertex { position: right_top, uv: Vec2f::new(1.0, 1.0), color: color },
		]);

		triangles.push([
			Vertex { position: left_bot, uv: Vec2f::new(0.0, 0.0), color: color },
			Vertex { position: right_top, uv: Vec2f::new(1.0, 1.0), color: color },
			Vertex { position: v(left_bot.x, right_top.y), uv: Vec2f::new(0.0, 1.0), color: color },
		]);
	}

	pub fn world(&mut self, tilemap: &TileMap, fluidmap: &FluidMap) {
		self.world = Some(GraphicsWorld::new(tilemap, fluidmap));
	}
}
