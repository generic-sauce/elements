window.onkeydown = function(ev) {
	if (!worker) { return; }

	worker.postMessage({
		type: "peripherals-event",
		ev: {
			peri_type: "keydown",
			key: ev.key,
		},
	});
}

window.onkeyup = function(ev) {
	if (!worker) { return; }

	worker.postMessage({
		type: "peripherals-event",
		ev: {
			peri_type: "keyup",
			key: ev.key,
		},
	});
}

window.onmousemove = function(ev) {
	if (!worker) { return; }

	worker.postMessage({
		type: "peripherals-event",
		ev: {
			peri_type: "mousemove",
			movement: [ev.movementX, ev.movementY],
		},
	});
}

window.onmousedown = function(ev) {
	if (!worker) { return; }

	worker.postMessage({
		type: "peripherals-event",
		ev: {
			peri_type: "mousedown",
			button: ev.button,
		},
	});
}

window.onmouseup = function(ev) {
	if (!worker) { return; }

	worker.postMessage({
		type: "peripherals-event",
		ev: {
			peri_type: "mouseup",
			button: ev.button,
		},
	});
}
