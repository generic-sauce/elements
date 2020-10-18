
// calculating the input states

export function get_input_states() {
	return [calc_input_state(0), calc_input_state(1)];
}

function calc_input_state(i) {
	const gp = navigator.getGamepads()[i];
	const last = e2.input_states[i];

	// fallback!
	if (!gp) {
		return e2.input_states[i];
	}

	const chrome = navigator.userAgent.toLowerCase().indexOf("chrom") != -1;

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
		var xi = 3;
		if (chrome) xi = 2;

		var yi = 4;
		if (chrome) yi = 3;

		const x = gp.axes[xi];
		const y = gp.axes[yi];

		cursor[0] = Math.floor(x * 2000.0);
		cursor[1] = Math.floor(-y * 2000.0);
	}

	const attack2 = gp.buttons[5].pressed;
	const just_attack2 = (!last.attack2) && attack2;

	var special1 = gp.axes[2] >= 0.1;
	if (chrome) special1 = gp.buttons[6].pressed;

	var attack1 = gp.axes[5] >= 0.1;
	if (chrome) attack1 = gp.buttons[7].pressed;

	var ret = {
		direction,
		cursor: cursor,
		just_up: false, // never read
		just_down: false, // never read
		special1,
		special2: false, // never read
		attack1,
		attack2,
		just_attack2: just_attack2,
	};

	e2.input_states[i] = ret;
	return ret;
}

// init

export function init() {
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
}
