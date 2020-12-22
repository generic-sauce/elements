export function handle_command(cmd) {
	if ("QueueMusic" in cmd) {
		queue_music(cmd["QueueMusic"])
	} else if ("PlaySound" in cmd) {
		play_sound(...cmd["PlaySound"])
	} else {
		console.log("invalid audio command!", cmd);
	}
}

var howl = null

function queue_music(file, volume) {
	const first_time = (howl == null);

	howl = new Howl({
		src: "res/" + file,
		volume,
		onend: () => howl.play(),
	})

	if (first_time) { howl.play(); }
}

function play_sound(file, volume) {
	new Howl({
		src: "res/" + file,
		volume,
	}).play()
}
