// calculating the input states

export function calc_input_state(i) {
	const gp = navigator.getGamepads()[i];

	// fallback!
	if (!gp) {
		return default_input_state();
	}

	const chrome = navigator.userAgent.toLowerCase().indexOf("chrom") != -1;

	const stick_left = [gp.axes[0], -gp.axes[1]];

	let stick_right;
	{
		let xi = 3;
		if (chrome) xi = 2;

		let yi = 4;
		if (chrome) yi = 3;

		stick_right = [gp.axes[xi], -gp.axes[yi]];
	}

	const bumper_right = gp.buttons[5].pressed;

	let trigger_left = gp.axes[2];
	if (chrome) trigger_left = gp.buttons[6].pressed * 1.0;

	let trigger_right = gp.axes[5];
	if (chrome) trigger_right = gp.buttons[7].pressed * 1.0;

	let dpad;
	{
		if (chrome) dpad = [gp.buttons[15].value - gp.buttons[14].value, gp.buttons[12].value - gp.buttons[13].value];
		else dpad = [gp.axes[6], -gp.axes[7]];
	}

	return {
		stick_left,
        stick_right,
		trigger_left,
		trigger_right,
		bumper_right,
		dpad,
        // TODO
		bumper_left: false,
		button_north: false,
		button_west: false,
		button_east: false,
		button_south: false,
	};
}

function default_input_state() {
	return {
		stick_left: [0, 0],
		stick_right: [0, 0],
		dpad: [0.0, 0.0],
		trigger_left: 0,
		trigger_right: 0,
		bumper_right: false,
		bumper_left: false,
		button_north: false,
		button_west: false,
		button_east: false,
		button_south: false,
	};
}
