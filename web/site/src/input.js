
// calculating the input states

export function get_input_states() {
	return [calc_input_state(0), calc_input_state(1)];
}

function calc_input_state(i) {
	const last = e2.input_states[i];
	const current = current_hardware_input_state(i);

	// TODO merge them
	return current;
}

function current_hardware_input_state(i) {
	const gp = e2.gamepads[i];

	// fallback!
	if (!gp) {
		return e2.input_states[i];
	}

	var direction = [0, 0];
	{
		const x = gp.axes[0];
		const y = gp.axes[1];

		if (x < -0.3) { direction[0] = -100; }
		if (x > 0.3) { direction[0] = 100; }

		if (y < -0.3) { direction[1] = 100; }
	}

	var cursor = [0, 0];
	{
		const x = gp.axes[3];
		const y = gp.axes[4];

		cursor[0] = Math.floor(x * 2000.0);
		cursor[1] = Math.floor(-y * 2000.0);
	}

	return {
		direction: direction,
		cursor: cursor,
		just_up: false,
		just_down: false,
		special1: false,
		special2: false,
		attack1: false,
		attack2: false,
		just_attack2: false,
	};
}

// init

export function init() {
	function gamepadHandler(ev, connecting) {
	  var gamepad = ev.gamepad;
	  // Note:
	  // gamepad === navigator.getGamepads()[gamepad.index]

	  if (connecting) {
		e2.gamepads[gamepad.index] = gamepad;
	  } else {
		delete e2.gamepads[gamepad.index];
	  }
	}

	function default_input_state() {
		return {
			direction: [0.0, 0.0],
			cursor: [0.0, 0.0],
			just_up: false,
			just_down: false,
			special1: false,
			special2: false,
			attack1: false,
			attack2: false,
			just_attack2: false,
		};
	}

	e2.input_states = [default_input_state(), default_input_state()];
	e2.gamepads = {};

	window.addEventListener("gamepadconnected", function(e) { gamepadHandler(e, true); }, false);
	window.addEventListener("gamepaddisconnected", function(e) { gamepadHandler(e, false); }, false);
}
