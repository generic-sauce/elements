
// calculating the input states

e2.get_input_states = function() {
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
	var r = 0;
	if (gp.axes[0] < -0.3) { r = -100; }
	if (gp.axes[0] > 0.3) { r = 100; }

	var u = 0;
	if (gp.axes[1] < -0.3) { u = 100; }

	return {
		direction: [r, u],
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

// init

function init_inputs() {
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

init_inputs();
