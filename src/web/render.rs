use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct WebJsonDraw {
	pub tilemap_size: TileVec,
}

pub struct WebDraw {
	pub json_draw: JsValue,
	pub tilemap_data: Uint8Array,
}

impl WebDraw {
	pub fn new(draw: Draw) -> WebDraw {
		let world = draw.world.unwrap();

		let tilemap_size = world.tilemap_size;
		let json_draw = WebJsonDraw {
			tilemap_size,
		};
		let json_draw = JsValue::from_serde(&json_draw).unwrap();

		let tilemap_data: Uint8Array = world.tilemap_data[..].into();

		WebDraw {
			json_draw,
			tilemap_data,
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
