export function handle_command(cmd) {
	if ("QueueMusic" in cmd) {
		queue_music(cmd["QueueMusic"])
	} else if ("PlaySound" in cmd) {
		play_sound(...cmd["PlaySound"])
	} else {
		console.log("invalid audio command!", cmd);
	}
}

var next_music = null

function start_music() {
	new Howl({
		src: next_music,
		onend: start_music,
	}).play()
}

function queue_music(file) {
	const abs_file = "res/" + file

	if (next_music) {
		next_music = abs_file
	} else {
		next_music = abs_file
		start_music()
	}
}

function play_sound(file, volume) {
	new Howl({
		src: "res/" + file,
		volume,
	}).play()
}
