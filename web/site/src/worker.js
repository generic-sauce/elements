self.input_states = [default_input_state(), default_input_state()];

self.input_state = function(i) {
	return self.input_states[i];
}

self.tilemap_load_callback = null
self.onmessage = function(e) {
	const msg = e.data;
	if (msg.type == "load_tilemap_response") {
		if (self.tilemap_load_callback) {
			self.tilemap_load_callback(msg.tilemap);
			self.tilemap_load_callback = null;
		} else {
			console.log("panic!");
		}
	} else if (msg.type == "input") {
		self.input_states = msg.states;
	} else {
		console.log("received invalid message at worker/mod.js", msg);
	}
}

self.load_tilemap = function(src, cb) {
	self.postMessage({
		type: "load_tilemap_request",
		filename: src,
	});
	self.tilemap_load_callback = cb;
}

self.js_init = function(texture_filenames) {
	self.postMessage({
		type: "init",
		texture_filenames,
	});
}

self.js_render = function(draw, tilemap_data, fluidmap_data, vertex_data) {
	draw.tilemap_data = tilemap_data
	draw.fluidmap_data = fluidmap_data
	draw.vertex_data = vertex_data

	postMessage({
		type: "render",
		draw,
	});
}

// TODO make non-redundant
function default_input_state() {
	return {
		stick_left: [0, 0],
		stick_right: [0, 0],
		dpad: [0.0, 0.0],
		trigger_left: 0,
		trigger_right: 0,
		bumper_right: false,
		bumper_left: false,
		button_north: false,
		button_west: false,
		button_east: false,
		button_south: false,
	};
}


import("../node_modules/elements/elements.js") // TODO use web/pkg-path without linking
