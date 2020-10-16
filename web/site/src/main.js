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

function init() {
	e2.world_occupied = false
	e2.rust.init();
	e2.cnst = e2.rust.constants();

	drawmod.init();
	inputmod.init();

	tilemapmod.load("map/map02.png", function(img) {
		e2.world_ptr = e2.rust.new_world(img);
		setInterval(meta_tick, 1000.0/60.0);
	})
}

function tick() {
	e2.rust.tick_world(e2.world_ptr, inputmod.get_input_states());
	e2.world = e2.rust.world_to_json(e2.world_ptr);
	drawmod.draw();
}

function meta_tick() {
	if (e2.world_occupied) {
		console.log("framedrop!");
		return;
	}

	e2.world_occupied = true;
	tick();
	e2.world_occupied = false;
}
