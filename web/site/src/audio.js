export function handle_command(cmd) {
	if ("QueueMusic" in cmd) {
		queue_music(cmd["QueueMusic"])
	} else if ("PlaySound" in cmd) {
		play_sound(...cmd["PlaySound"])
	}
}

function queue_music(file) {
	console.log("TODO: queue music!");
}

function play_sound(file, volume) {
	console.log("TODO: play sound " + file + " " + volume);
}
