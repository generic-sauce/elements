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

	e2.socket = new WebSocket("ws://127.0.0.1:7575");

	e2.webclient = e2.rust.new_webclient();

	e2.socket.onerror = function(e) {
		console.log("socket error:", e);
	}

	e2.socket.onmessage = function(e) {
		e2.rust.webclient_received_message(e2.webclient, e.data);
	}

	tilemapmod.load("map/map02.png", function(img) {
		e2.world_ptr = null;
		setInterval(tick, 1000.0/FPS);
	})
}

function tick() {
	while (fps() < FPS) {
		e2.rust.webclient_tick(e2.webclient, inputmod.get_input_states())
			.forEach(cmd => {
				if (cmd.SendMsg) {
					e2.socket.send(cmd.SendMsg.msg);
				} else if (cmd.Go) {
					e2.world_ptr = cmd.Go.world;
				} else {
					alert("unknown cmd!");
				}
			});
		e2.frame_counter += 1;

		// TODO send input-state packet
	}

	if (e2.world_ptr) {
		e2.render_world = e2.rust.to_render_world(e2.world_ptr);
		e2.render_world.tilemap_data = e2.rust.tilemap_data(e2.world_ptr);
		e2.render_world.fluidmap_data = e2.rust.fluidmap_data(e2.world_ptr);

		drawmod.draw();
	}
}
