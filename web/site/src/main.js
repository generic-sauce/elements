// the object storing the state of our application
window.e2 = {};

import("./draw/mod.js");
import("./input.js");

function init() {
	e2.world_occupied = false
	e2.rust.init();
	e2.cnst = e2.rust.constants();
	e2.init_drawing();
	e2.world_ptr = e2.rust.new_world();
}

function tick() {
	e2.rust.tick_world(e2.world_ptr, e2.get_input_states());
	e2.world = e2.rust.world_to_json(e2.world_ptr);
	e2.draw_world();
}

function meta_tick() {
	if (e2.world_occupied) {
		alert("framedrop!");
		return;
	}

	e2.world_occupied = true;
	tick();
	e2.world_occupied = false;
}

const js = import("../node_modules/elements2/elements2.js");
js.then(rust => {
	e2.rust = rust;

	init();

	setInterval(meta_tick, 1000.0/60.0);
});
