import "./draw.js"

// TODO nice main-loop
function schedule(local, js) {
	js.work_local(local);
	const re_schedule = function() { schedule(local, js); }
	setTimeout(re_schedule, 16);
}

const js = import("./node_modules/elements2/elements2.js");
js.then(js => {
	var local = js.init();
	schedule(local, js);
});
