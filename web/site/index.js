// TODO nice main-loop
function schedule(w, js) {

	js.tick_world_nohandler(w);
	console.log(js.world_to_json(w).players[0].left_bot);

	const re_schedule = function() { schedule(w, js); }
	setTimeout(re_schedule, 16);
}

const js = import("./node_modules/elements2/elements2.js");
js.then(js => {
	js.init();
	const w = js.new_world();
	schedule(w, js);
});
