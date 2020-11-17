use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct JsWebRenderDrawTilemap {
	pub size: TileVec,
	pub depth_value: DepthValue,
}

#[derive(Serialize, Deserialize)]
pub struct JsWebRenderDrawFluidmap {
	pub size: FluidVec,
	pub depth_value: DepthValue,
}

#[derive(Serialize, Deserialize)]
pub struct WebText {
	pub left_bot: SurfaceVec,
	pub scale: f32,
	pub color: Color,
	pub string: String,
}

#[derive(Serialize, Deserialize)]
pub struct JsWebRenderDraw {
	pub clear_color: Color,
	pub vertex_counts: Vec<u32>, // number of vertices per texture
	pub tilemap: Option<JsWebRenderDrawTilemap>,
	pub fluidmap: Option<JsWebRenderDrawFluidmap>,
	pub texts: Vec<WebText>,
}

pub struct WebRenderDraw {
	pub js_web_render_draw: JsValue,
	pub vertex_data: Uint8Array, // vertices for all textures in bytes
	pub tilemap_data: Uint8Array,
	pub fluidmap_data: Uint8Array,
}

impl WebRenderDraw {
	pub fn new(draw: Draw) -> WebRenderDraw {
		// canvas vec cast because we only need aspect. kinda shady
		let render_draw = RenderDraw::new(draw, CanvasVec::aspect().cast());
		let RenderDraw { clear_color, vertex_data, vertex_counts, texts, tilemap, fluidmap } = render_draw;

		let (tilemap, tilemap_data) = match tilemap {
			Some(RenderDrawTilemap { data, size, depth_value }) => (
				Some(JsWebRenderDrawTilemap { size, depth_value }),
				data[..].into()
			),
			None => (None, Uint8Array::new_with_length(0))
		};

		let (fluidmap, fluidmap_data) = match fluidmap {
			Some(RenderDrawFluidmap { data, size, depth_value }) => (
				Some(JsWebRenderDrawFluidmap { size, depth_value }),
				data[..].into()
			),
			None => (None, Uint8Array::new_with_length(0))
		};

		let mut web_texts = Vec::new();
		web_texts.extend(
			texts.into_iter()
				.map(move |text| WebText {
					left_bot: text.left_bot.to_surface_correct_aspect(),
					scale: text.scale,
					color: text.color,
					string: text.string,
				})
		);

		let js_web_render_draw = JsWebRenderDraw {
			clear_color,
			vertex_counts,
			tilemap,
			fluidmap,
			texts: web_texts,
		};
		let js_web_render_draw = JsValue::from_serde(&js_web_render_draw).unwrap();

		let vertex_data: Uint8Array = vertex_data[..].into();

		WebRenderDraw {
			js_web_render_draw,
			vertex_data,
			tilemap_data,
			fluidmap_data,
		}
	}
}

// #[derive(Serialize, Deserialize)]
// pub struct RenderWorld {
// 	tilemap_size: TileVec,
// 	fluidmap_size: FluidVec,
// 	players: [Player; 2],
// 	player_size: GameVec,
//
// 	// in javascript this object has the additional members:
// 	// tilemap_data: Uint8Array,
// 	// fluidmap_data: Uint8Array,
// }
//
// impl RenderWorld {
// 	pub fn draw(w: &World) {
// 		draw_render_world(
// 			World::render_world(w),
// 			World::tilemap_data(w),
// 			World::fluidmap_data(w)
// 		);
// 	}
// }
//
// impl World {
// 	pub fn render_world(w: &World) -> JsValue {
// 		let tilemap_size = w.tilemap.size;
// 		let fluidmap_size = w.fluidmap.size;
//
// 		let rw = RenderWorld {
// 			tilemap_size,
// 			fluidmap_size,
// 			players: w.players.clone(),
// 			player_size: PLAYER_SIZE,
// 		};
// 		JsValue::from_serde(&rw).unwrap()
// 	}
//
// 	pub fn tilemap_data(w: &World) -> Uint8Array {
// 		let mut tilemap_data = Vec::new();
//
// 		for y in 0..w.tilemap.size.y {
// 			for x in 0..w.tilemap.size.x {
// 				let color: [u8; 4] =
// 					match w.tilemap.get(TileVec::new(x, y)) {
// 						Tile::Ground => [50, 50, 10, 255],
// 						Tile::Void => [0, 0, 0, 0],
// 						Tile::Wall { owner: 0, .. } => [0, 0, 40, 255],
// 						Tile::Wall { owner: 1, .. } => [40, 0, 0, 255],
// 						Tile::Wall { owner: _, .. } => panic!("more than two players are not yet supported"),
// 					};
// 				tilemap_data.extend(&color[..]);
// 			}
// 		}
//
// 		(&tilemap_data[..]).into()
// 	}
//
// 	pub fn fluidmap_data(w: &World) -> Uint8Array {
// 		let mut fluidmap_data = Vec::new();
//
// 		for y in 0..w.fluidmap.size.y {
// 			for x in 0..w.fluidmap.size.x {
// 				let fluids = w.fluidmap.index(FluidVec::new(x, y));
// 				let mut color: [u8; 4] = [0; 4];
// 				for f in fluids {
// 					color[3] = 255;
// 					if f.owner == 0 {
// 						color[2] = 255;
// 					} else if f.owner == 1 {
// 						color[0] = 255;
// 					} else {
// 						panic!("more than two players are not yet supported (2)");
// 					}
// 				}
// 				fluidmap_data.extend(&color[..]);
// 			}
// 		}
//
// 		(&fluidmap_data[..]).into()
// 	}
//
// }
