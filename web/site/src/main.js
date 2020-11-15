import * as render_mod from "./render/mod.js"
import * as tilemapmod from "./tilemap.js"
import * as inputmod from "./input.js"

window.draw = null
window.worker = new Worker("./src/worker.js")
window.worker.onmessage = function(e) {
	const msg = e.data;

	if (msg.type == "init") {
		render_mod.init(msg.texture_filenames);
	} else if (msg.type == "render") {
		window.draw = msg.draw;
	} else if (msg.type == "load_tilemap_request") {
		tilemapmod.load(msg.filename, function(tilemap) {
			window.worker.postMessage({
				type: "load_tilemap_response",
				tilemap,
			});
		});
	} else {
		console.log("invalid message received at main.js", msg)
	}
}

setInterval(function() {
	if (window.draw) {
		render_mod.render(window.draw)
	}
}, 1000/60)

setInterval(function() {
	window.worker.postMessage({
		type: "input",
		states: [inputmod.calc_input_state(0), inputmod.calc_input_state(1)],
	});
}, 1000/20)
