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
	e2.rust.tick_world(e2.world_ptr, inputmod.get_input_states());
	e2.render_world = e2.rust.to_render_world(e2.world_ptr);
	drawmod.draw();
}

function meta_tick() {
	const frame_start = new Date();

	tick();

	const frame_time = new Date() - frame_start;

	if (frame_time >= 1000.0/FPS) {
		console.log("frame took too long: ", frame_time);
	}

}
