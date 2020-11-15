import * as render_mod from "./render/mod.js"

window.draw = null
window.worker = new Worker("./src/worker/mod.js")
window.worker.onmessage = function(e) {
	const msg = e.data;

	if (msg.type == "init") {
		render_mod.init(msg.texture_filenames);
	} else if (msg.type == "render") {
		window.draw = msg.draw;
	} else {
		console.log("invalid message received at main.js", msg)
	}
}

setInterval(function() {
	if (window.draw) {
		render_mod.render(window.draw)
	}
}, 1000/60)
