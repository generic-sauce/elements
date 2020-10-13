// TODO nice main-loop
function schedule(local, js) {

	// TODO call wasm-tick here

	const re_schedule = function() { schedule(local, js); }
	setTimeout(re_schedule, 16);
}

const js = import("./node_modules/elements2/elements2.js");
js.then(js => {
	const w = js.new_world();
	alert(w);
	const jsonw = js.world_to_json(w);
	alert(jsonw);
	schedule(new Object(), js);
});
