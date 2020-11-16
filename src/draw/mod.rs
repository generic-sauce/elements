use crate::prelude::*;

mod player;
use player::*;

mod world;
pub use world::*;

mod texture;
pub use texture::*;

mod vec;
pub use vec::*;

mod color;
pub use color::*;

mod render;
pub use render::*;

mod lobby;
pub use lobby::*;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Flip {
	Normal,
	Horizontal,
}

pub trait IntoTextureIndex {
	fn into_texture_index(self) -> usize;
}

pub struct Draw {
	clear_color: Option<Color>,
	depth_index: DepthIndex,
	texture_triangles: TextureTriangles,
	tilemap: Option<DrawTilemap>,
	fluidmap: Option<DrawFluidmap>,
	texts: Vec<Text>,
}

impl Draw {
	pub fn new() -> Draw {
		let clear_color = None;
		let depth_index = 0.0;
		let mut texture_triangles = TextureTriangles::new();
		texture_triangles.resize_with(TextureId::texture_count(), Default::default);
		let texts = Vec::new();
		let tilemap = None;
		let fluidmap = None;

		Draw {
			clear_color,
			depth_index,
			texture_triangles,
			texts,
			tilemap,
			fluidmap,
		}
	}

	pub fn set_clear_color(&mut self, clear_color: Color) {
		if let Some(_) = self.clear_color {
			panic!("clear color was set already");
		}
		self.clear_color = Some(clear_color);
	}

	#[allow(unused)]
	pub fn texture(
		&mut self,
		left_bot: impl IntoViewVec,
		right_top: impl IntoViewVec,
		texture_index: impl IntoTextureIndex,
		flip: Flip,
		color: Option<Color>,
	) {
		let texture_index = texture_index.into_texture_index();
		let triangles = &mut self.texture_triangles[texture_index];
		let left_bot = left_bot.to_view();
		let right_top = right_top.to_view();
		let color = color.unwrap_or(Color::WHITE);
		let (left_uv, right_uv) = match flip {
			Flip::Normal => (0.0, 1.0),
			Flip::Horizontal => (1.0, 0.0),
		};

		triangles.push(Triangle {
			vertices: [
				Vertex { position: left_bot,                   uv: TextureVec::new(left_uv, 0.0),  color },
				Vertex { position: v(right_top.x, left_bot.y), uv: TextureVec::new(right_uv, 0.0), color },
				Vertex { position: right_top,                  uv: TextureVec::new(right_uv, 1.0), color },
			],
			depth_index: self.depth_index,
		});

		triangles.push(Triangle {
			vertices: [
				Vertex { position: left_bot,                   uv: TextureVec::new(left_uv, 0.0),  color },
				Vertex { position: right_top,                  uv: TextureVec::new(right_uv, 1.0), color },
				Vertex { position: v(left_bot.x, right_top.y), uv: TextureVec::new(left_uv, 1.0),  color },
			],
			depth_index: self.depth_index,
		});

		self.depth_index += 1.0;
	}

	#[allow(unused)]
	pub fn rectangle(
		&mut self,
		left_bot: impl IntoViewVec,
		right_top: impl IntoViewVec,
		color: Color,
	) {
		let triangles = &mut self.texture_triangles[TextureId::White as usize];
		let left_bot = left_bot.to_view();
		let right_top = right_top.to_view();

		triangles.push(Triangle {
			vertices: [
				Vertex { position: left_bot,                   uv: TextureVec::new(0.0, 0.0), color },
				Vertex { position: v(right_top.x, left_bot.y), uv: TextureVec::new(1.0, 0.0), color },
				Vertex { position: right_top,                  uv: TextureVec::new(1.0, 1.0), color },
			],
			depth_index: self.depth_index,
		});

		triangles.push(Triangle {
			vertices: [
				Vertex { position: left_bot,                   uv: TextureVec::new(0.0, 0.0), color },
				Vertex { position: right_top,                  uv: TextureVec::new(1.0, 1.0), color },
				Vertex { position: v(left_bot.x, right_top.y), uv: TextureVec::new(0.0, 1.0), color },
			],
			depth_index: self.depth_index,
		});

		self.depth_index += 1.0;
	}

	pub fn map(&mut self, tilemap: &TileMap, fluidmap: &FluidMap) {
		// draw and render order have to be changed respectively
		self.tilemap = Some(DrawTilemap::new(tilemap, self.depth_index + 1.0));
		self.fluidmap = Some(DrawFluidmap::new(fluidmap, self.depth_index));

		self.depth_index += 2.0;
	}

	fn circle_frac_iter(points: u32) -> impl Iterator<Item=(f32, f32)> {
		let frac = std::f32::consts::PI * 2.0 / points as f32;
		let i0 = (0..points)
			.map(move |i| frac * i as f32);
		let i1 = (0..points)
			.map(move |i| frac * (i+1) as f32);

		i0.zip(i1)
	}

	pub fn circle(
		&mut self,
		center: impl IntoViewVec,
		scale: f32,
		color: Color,
	) {
		self.arc(center, scale, color, 1.0, 0.0);
	}

	pub fn arc(
		&mut self,
		center: impl IntoViewVec,
		scale: f32,
		color: Color,
		arc_size: f32,
		arc_offset: f32,
	) {
		let points = 32;
		let center = center.to_view();
		let center_uv = TextureVec::new(0.5, 0.5);
		let arc_offset = arc_offset * std::f32::consts::PI * 2.0;
		let triangles = &mut self.texture_triangles[TextureId::White as usize];

		for (frac0, frac1) in Self::circle_frac_iter(points) {
			let frac0 = frac0 * arc_size + arc_offset;
			let frac1 = frac1 * arc_size + arc_offset;

			let x0 = f32::cos(frac0);
			let y0 = f32::sin(frac0);
			let x1 = f32::cos(frac1);
			let y1 = f32::sin(frac1);

			let point0 = center + CanvasVec::new(x0, y0).to_view() * scale;
			let point1 = center + CanvasVec::new(x1, y1).to_view() * scale;
			let uv0 = TextureVec::new(x0, y0) * 0.5 + 0.5;
			let uv1 = TextureVec::new(x1, y1) * 0.5 + 0.5;

			triangles.push(Triangle {
				vertices: [
					Vertex { position: center, uv: center_uv, color },
					Vertex { position: point0, uv: uv0,       color },
					Vertex { position: point1, uv: uv1,       color },
				],
				depth_index: self.depth_index,
			});
		}

		self.depth_index += 1.0;
	}

	#[allow(unused)]
	pub fn text(
		&mut self,
		left_bot: impl IntoViewVec,
		scale: f32,
		color: Color,
		string: &str,
	) {
		let left_bot = left_bot.to_view();
		let string = string.to_string();

		let text = Text {
			left_bot,
			scale,
			color,
			string,
		};

		self.texts.push(text);
	}
}

struct Vertex {
	position: ViewVec,
	uv: TextureVec,
	color: Color,
}

// index of draw command recorded by draw
type DepthIndex = f32;

// vertex depth used for actual rendering
pub type DepthValue = f32;

struct Triangle {
	vertices: [Vertex; 3],
	depth_index: DepthIndex,
}

type Triangles = Vec<Triangle>;
type TextureTriangles = Vec<Triangles>;

pub struct Text {
	pub left_bot: ViewVec,
	pub scale: f32,
	pub color: Color,
	pub string: String,
}

struct DrawTilemap {
	size: TileVec,
	data: Vec<u8>,
	depth_index: DepthIndex,
}

impl DrawTilemap {
	fn new(tilemap: &TileMap, depth_index: DepthIndex) -> DrawTilemap {
		let size = tilemap.size;
		let data: Vec<u8> = tilemap.iter()
			.map(|p| tilemap.get(p))
			.map(|t| match t {
				Tile::Void => 0,
				Tile::Ground => 1,
				Tile::Wall { owner, .. } => 2 + owner as u8,
			})
			.collect();

		DrawTilemap {
			size,
			data,
			depth_index,
		}
	}
}

struct DrawFluidmap {
	size: FluidVec,
	data: Vec<u8>,
	depth_index: DepthIndex,
}

impl DrawFluidmap {
	fn new(fluidmap: &FluidMap, depth_index: DepthIndex) -> DrawFluidmap {
		let size: FluidVec = TileVec::new(128, 72).cast();

		let mut data: Vec<u8> = Vec::new();
		data.resize((4 * size.x * size.y) as usize, 0 as u8);

		for fluid in fluidmap.iter() {
			let cell_id = fluid.position / TILESIZE;
			let local_position = ((fluid.position.x % TILESIZE) as u8, (fluid.position.y % TILESIZE) as u8);

			let cell_index = 4 * (cell_id.x + cell_id.y * size.x as i32) as usize;
			data[cell_index+3] = 255;
			data[cell_index+2] = (fluid.owner * 255) as u8;
			data[cell_index+1] = local_position.1 as u8;
			data[cell_index]   = local_position.0 as u8;
		}

		DrawFluidmap {
			size,
			data,
			depth_index,
		}
	}
}
