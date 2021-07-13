import * as render_mod from "./render/mod.js"
import * as tilemapmod from "./tilemap.js"
import * as gamepadmod from "./gamepad.js"
import * as peripheralsmod from "./peripherals.js"
import * as audiomod from "./audio.js"

window.onload = function() {
	document.addEventListener("click", function () {
		document.body.requestPointerLock();
	});

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
		} else if (msg.type == "audio-command") {
			audiomod.handle_command(msg.cmd)
		} else if (msg.type == "set_localstorage") {
			localStorage.setItem(msg.key, msg.value);
		} else {
			console.log("invalid message received at main.js", msg)
		}
	}

	function send_gamepad_update() {
		window.worker.postMessage({
			type: "gamepad-update",
			states: [gamepadmod.calc_gamepad_state(0), gamepadmod.calc_gamepad_state(1)],
		});
	}

	function draw_fn() {
		send_gamepad_update()
		if (window.draw) {
			render_mod.render(window.draw)
		}
		send_gamepad_update()

		requestAnimationFrame(draw_fn)
	}

	requestAnimationFrame(draw_fn)

	setInterval(send_gamepad_update, 1000/120)

	// init localstorage
	var data = {};
	for (var i = 0; i < localStorage.length; i++){
		const key = localStorage.key(i);
		data[key] = localStorage.getItem(key);
	}

	window.worker.postMessage({
		type: "init_localstorage",
		data,
	});
}
