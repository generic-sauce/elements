use crate::prelude::*;

use js_sys::Uint8Array;

#[derive(Serialize, Deserialize)]
pub struct RenderWorld {
	tilemap_size: TileVec,
	fluidmap_size: FluidVec,
	players: [Player; 2],
	player_size: GameVec,

	// in javascript this object has the additional members:
	// tilemap_data: Uint8Array,
	// fluidmap_data: Uint8Array,
}

#[wasm_bindgen]
pub fn to_render_world(w: *const World) -> JsValue {
	let w = unsafe { & *w };

	let tilemap_size = w.tilemap.size;
	let fluidmap_size = w.fluidmap.size;

	let rw = RenderWorld {
		tilemap_size,
		fluidmap_size,
		players: w.players.clone(),
		player_size: PLAYER_SIZE,
	};
	JsValue::from_serde(&rw).unwrap()
}

#[wasm_bindgen]
pub fn tilemap_data(w: *const World) -> Uint8Array {
	let w = unsafe { & *w };

	let mut tilemap_data = Vec::new();

	for y in 0..w.tilemap.size.y {
		for x in 0..w.tilemap.size.x {
			let color: [u8; 4] =
				match w.tilemap.get(TileVec::new(x, y)) {
					Tile::Ground => [50, 50, 10, 255],
					Tile::Void => [200, 200, 255, 255],
					Tile::Wall { owner: 0, .. } => [0, 0, 40, 255],
					Tile::Wall { owner: 1, .. } => [40, 0, 0, 255],
					Tile::Wall { owner: _, .. } => panic!("more than two players are not yet supported"),
				};
			tilemap_data.extend(&color[..]);
		}
	}

	(&tilemap_data[..]).into()
}


#[wasm_bindgen]
pub fn fluidmap_data(w: *const World) -> Uint8Array {
	let w = unsafe { & *w };

	let mut fluidmap_data = Vec::new();

	for y in 0..w.fluidmap.size.y {
		for x in 0..w.fluidmap.size.x {
			let fluids = w.fluidmap.index(FluidVec::new(x, y));
			let mut color: [u8; 4] = [0; 4];
			for f in fluids {
				color[3] = 255;
				if f.owner == 0 {
					color[2] = 255;
				} else if f.owner == 1 {
					color[0] = 255;
				} else {
					panic!("more than two players are not yet supported (2)");
				}
			}
			fluidmap_data.extend(&color[..]);
		}
	}

	(&fluidmap_data[..]).into()
}
