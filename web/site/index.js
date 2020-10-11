import "./draw.js";

window.keys = {W: false, S: false, A: false, D: false};


window.onload = function() {
	document.onkeydown = function(evt) {
		evt = evt || window.event;
		const charCode = evt.keyCode || evt.which;
		const s = String.fromCharCode(charCode);
		window.keys[s] = true;
	};

	document.onkeyup = function(evt) {
		evt = evt || window.event;
		const charCode = evt.keyCode || evt.which;
		const s = String.fromCharCode(charCode);
		window.keys[s] = false;
	};
};

window.get_wasd = function() {
	return ["W", "A", "S", "D"].map(x => window.keys[x]);
}

// TODO nice main-loop
function schedule(local, js) {
	js.work_local(local);
	const re_schedule = function() { schedule(local, js); }
	setTimeout(re_schedule, 16);
}

const js = import("./node_modules/elements2/elements2.js");
js.then(js => {
	var local = js.init();
	init_drawing();
	schedule(local, js);
});
