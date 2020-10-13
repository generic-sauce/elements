e2.init_input = function() {
	e2.input_states = [default_input_state(), default_input_state()];
}

function default_input_state() {
	return {
		direction: [0.0, 100.0],
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
