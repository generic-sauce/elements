// TODO nice main-loop
function schedule(local, js) {

	// TODO call wasm-tick here

	const re_schedule = function() { schedule(local, js); }
	setTimeout(re_schedule, 16);
}

const js = import("./node_modules/elements2/elements2.js");
js.then(js => {
	var local = js.init();
	schedule(local, js);
});
