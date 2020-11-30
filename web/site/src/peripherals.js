window.onkeydown = function(ev) {
	if (!worker) { return; }

	if (ev.key == "a") {
		worker.postMessage({
			type: "peripherals-event",
			ev: {
				peri_type: "keydown",
				key: "a",
			},
		});
	}
}

window.onkeyup = function(ev) {
	if (!worker) { return; }

	if (ev.key == "a") {
		worker.postMessage({
			type: "peripherals-event",
			ev: {
				peri_type: "keyup",
				key: "a",
			},
		});
	}
}
