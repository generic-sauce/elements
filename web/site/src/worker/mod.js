import * as inputmod from "./input.js"

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
	} else {
		console.log("received invalid message at worker/mod.js", msg);
	}
}

self.js_init = function(texture_filenames) {
	console.log("herewego:",texture_filenames);
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

self.input_state = function(i) {
	return inputmod.calc_input_state(i)
}

self.load_tilemap = function(src, cb) {
	self.postMessage({
		type: "load_tilemap_request",
		filename: src,
	});
	self.tilemap_load_callback = cb;
}

import("../../node_modules/elements/elements.js") // TODO use web/pkg-path without linking
