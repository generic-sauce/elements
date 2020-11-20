import * as render_mod from "./render/mod.js"
import * as tilemapmod from "./tilemap.js"
import * as inputmod from "./input.js"

const answer = prompt("menu / local / ip")

window.draw = null
window.worker = new Worker("./src/worker.js")
window.worker.onmessage = function(e) {
	const msg = e.data;

	if (msg.type == "init-response") {
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
window.worker.postMessage({
	type: "init-request",
	answer
});

function send_input_update() {
	window.worker.postMessage({
		type: "input",
		states: [inputmod.calc_input_state(0), inputmod.calc_input_state(1)],
	});
}

function draw_fn() {
	send_input_update()
	if (window.draw) {
		render_mod.render(window.draw)
	}
	send_input_update()

	requestAnimationFrame(draw_fn)
}

requestAnimationFrame(draw_fn)

setInterval(send_input_update, 1000/120)
