// the object storing the state of our application
window.e2 = {
	frame_counter: 0,
	program_start_time: performance.now(),
};

const FPS = 60.0;

import * as drawmod from "./draw/mod.js";
import * as inputmod from "./input.js";
import * as tilemapmod from "./tilemap.js";
import("../node_modules/elements2/elements2.js")
	.then(rust => {
		e2.rust = rust;
		init();
	});

function fps() {
	return e2.frame_counter * 1000 / (performance.now() - e2.program_start_time);
}

function init() {
	e2.rust.init();

	drawmod.init();
	inputmod.init();

	tilemapmod.load("map/map02.png", function(img) {
		e2.world_ptr = e2.rust.new_world(img);
		setInterval(tick, 1000.0/FPS);
	})
}

function tick() {
	while (fps() < FPS) {
		e2.rust.tick_world(e2.world_ptr, inputmod.get_input_states());
		e2.frame_counter += 1;
	}

	e2.render_world = e2.rust.to_render_world(e2.world_ptr);
	e2.render_world.tilemap_data = e2.rust.tilemap_data(e2.world_ptr);
	e2.render_world.fluidmap_data = e2.rust.fluidmap_data(e2.world_ptr);

	drawmod.draw();
}
