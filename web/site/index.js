// the object storing the state of our application
window.e2 = {};

import("./draw/mod.js");

function schedule() {
	e2.rust.tick_world_nohandler(e2.world_ptr);
	e2.world = e2.rust.world_to_json(e2.world_ptr);
	e2.draw_world();

	setTimeout(schedule, 16);
}

const js = import("./node_modules/elements2/elements2.js");
js.then(rust => {
	e2.rust = rust;

	e2.rust.init();
	e2.init_drawing();
	e2.world_ptr = e2.rust.new_world();

	schedule();
});
