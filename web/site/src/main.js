// the object storing the state of our application
window.e2 = {};

import * as drawmod from "./draw/mod.js";
import * as inputmod from "./input.js";
import * as tilemapmod from "./tilemap.js";
import("../node_modules/elements2/elements2.js")
	.then(rust => {
		e2.rust = rust;
		init();
	});

const FPS = 60.0;
var last_tick = performance.now();
var frame_counter = 0;
const program_start_time = performance.now();

function init() {
	e2.rust.init();

	drawmod.init();
	inputmod.init();

	tilemapmod.load("map/map02.png", function(img) {
		e2.world_ptr = e2.rust.new_world(img);
		setInterval(meta_tick, 1000.0/FPS);
	})
}

function tick() {
	frame_counter += 1;
	const a = performance.now();

	e2.rust.tick_world(e2.world_ptr, inputmod.get_input_states());

	const b = performance.now();

	e2.render_world = e2.rust.to_render_world(e2.world_ptr);
	e2.render_world.tilemap_data = e2.rust.tilemap_data(e2.world_ptr);
	e2.render_world.fluidmap_data = e2.rust.fluidmap_data(e2.world_ptr);

	const c = performance.now();

	drawmod.draw();

	const d = performance.now();

	const fps = frame_counter * 1000 / (a - program_start_time);

	const ret = {
		fps,
		tick: b - a,
		render_world: c - b,
		draw: d - c,
		sum: d - a,
		all: a - last_tick,
	};
	last_tick = a;
	return ret;
}

function meta_tick() {
	const timestats = tick();

	if (timestats.all >= 1000.0/FPS) {
		console.log("frame took too long:", timestats);
	}
}

