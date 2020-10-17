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
	const a = new Date();
	e2.rust.tick_world(e2.world_ptr, inputmod.get_input_states());
	const b = new Date();
	e2.render_world = e2.rust.to_render_world(e2.world_ptr);
	const c = new Date();
	drawmod.draw();
	const d = new Date();

	return {
		tick: b - a,
		render_world: c - b,
		draw: d - c,
		sum: d - a,
	};
}

function meta_tick() {
	const timestats = tick();

	if (timestats.sum >= 1000.0/FPS) {
		console.log("frame took too long:", timestats);
	}
}
