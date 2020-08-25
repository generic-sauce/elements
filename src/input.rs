use crate::prelude::*;
use gilrs::GamepadId;

const CONTROLLER_MAX: f32 = 1.0;
const DEADZONE_MIN: f32 = 0.35;
const CURSOR_DEADZONE: f32 = 0.05;
const DEADZONE_MAX: f32 = CONTROLLER_MAX - DEADZONE_MIN;
const MAX_MOVEMENT_VALUE: i32 = 100;
const JOYSTICK_DISTANCE: i32 = 2300;

fn apply_deadzone_min(value: f32, deadzone_min: f32) -> f32 {
	let sign = value.signum();
	let deadzone_max = 1.0 - deadzone_min;
	if value.abs() < deadzone_min {
		0.0
	} else if value.abs() > deadzone_max {
		sign
	} else {
		(value.abs() - deadzone_min) / (deadzone_max-deadzone_min) * sign
	}
}

fn apply_deadzone(value: f32) -> f32 {
	apply_deadzone_min(value, DEADZONE_MIN)
}

pub trait Input {
	fn horizontal_dir(&self) -> i32;
	fn vertical_dir(&self) -> i32;

	fn up(&self) -> bool { self.vertical_dir() >= (DEADZONE_MIN * MAX_MOVEMENT_VALUE as f32) as i32 }
	fn just_up(&self) -> bool;
	fn down(&self) -> bool { self.vertical_dir() <= (-DEADZONE_MIN * MAX_MOVEMENT_VALUE as f32) as i32 }
	fn just_down(&self) -> bool;
	fn right(&self) -> bool { self.horizontal_dir() >= (DEADZONE_MIN * MAX_MOVEMENT_VALUE as f32) as i32 }
	fn left(&self) -> bool { self.horizontal_dir() <= (-DEADZONE_MIN * MAX_MOVEMENT_VALUE as f32) as i32 }
	fn attack1(&self) -> bool;
	fn attack2(&self) -> bool;
	fn special1(&self) -> bool;
	fn special2(&self) -> bool;

	fn cursor(&self) -> GameVec;

	fn update(&mut self, player: &Player, gilrs: &gilrs::Gilrs);
}

// TODO: use bitmask instead of booleans
pub struct AdaptiveInput {
	index: u32,

	direction: GameVec,
	just_up: bool,
	just_down: bool,
	special1: bool,
	special2: bool,
	attack1: bool,
	attack2: bool,
	cursor: GameVec,
	gamepad_id: Option<GamepadId>,
	mouse_position: Vec2i,
}

fn get_gamepad(index: u32, gilrs: &gilrs::Gilrs) -> Option<GamepadId> {
	gilrs.gamepads()
		.map(|(gamepad_id, _)| gamepad_id)
		.find(|gamepad_id| Into::<usize>::into(*gamepad_id) == index as usize)
}

fn get_mouse_position() -> Vec2i {
	let mp = sfml::window::mouse::desktop_position();
	Vec2i::new(mp.x, mp.y)
}

impl AdaptiveInput {
	pub fn new(index: u32, gilrs: &gilrs::Gilrs) -> AdaptiveInput {
		AdaptiveInput {
			index,
			direction: GameVec::new(0, 0),
			just_up: false,
			just_down: false,
			special1: false,
			special2: false,
			attack1: false,
			attack2: false,
			cursor: GameVec::new(0, 0),
			gamepad_id: get_gamepad(index, gilrs),
			mouse_position: get_mouse_position(),
		}
	}
}

impl Input for AdaptiveInput {
	fn horizontal_dir(&self) -> i32 {
		self.direction.x
	}

	fn vertical_dir(&self) -> i32 {
		self.direction.y
	}

	fn just_up(&self) -> bool {
		self.just_up
	}

	fn just_down(&self) -> bool {
		self.just_down
	}

	fn attack1(&self) -> bool {
		self.attack1
	}

	fn attack2(&self) -> bool {
		self.attack2
	}

	fn special1(&self) -> bool {
		self.special1
	}

	fn special2(&self) -> bool {
		self.special2
	}

	fn cursor(&self) -> GameVec {
		self.cursor
	}

	fn update(&mut self, _player: &Player, gilrs: &gilrs::Gilrs) {
		let gamepad = self.gamepad_id.map(|x| gilrs.gamepad(x));
		let has_keyboard = self.index == 1;

		use sfml::window::Key::*;

		let last_frame_up = self.up();
		let last_frame_down = self.down();

		self.direction.x = 0;
		self.direction.y = 0;

		self.direction.y += if W.is_pressed() && has_keyboard {
			MAX_MOVEMENT_VALUE
		} else if S.is_pressed() && has_keyboard {
			-MAX_MOVEMENT_VALUE
		} else if let Some(gamepad) = gamepad {
			(apply_deadzone(gamepad.value(gilrs::Axis::LeftStickY)) * MAX_MOVEMENT_VALUE as f32) as i32
		} else {
			0
		};

		let mut key_pressed = false;
		if D.is_pressed() && has_keyboard {
			self.direction.x += MAX_MOVEMENT_VALUE;
			key_pressed = true;
		}
		if A.is_pressed() && has_keyboard {
			self.direction.x -= MAX_MOVEMENT_VALUE;
			key_pressed = true;
		}
		if !key_pressed {
			if let Some(gamepad) = gamepad {
				self.direction.x = (apply_deadzone(gamepad.value(gilrs::Axis::LeftStickX)) * MAX_MOVEMENT_VALUE as f32) as i32
			}
		}

		self.just_up = !last_frame_up && self.up();
		self.just_down = !last_frame_down && self.down();

		self.attack1 = Q.is_pressed() && has_keyboard;
		self.attack2 = E.is_pressed() && has_keyboard;
		self.special1 = R.is_pressed() && has_keyboard;
		self.special2 = F.is_pressed() && has_keyboard;

		if let Some(gamepad) = gamepad {
			let cx = gamepad.value(gilrs::Axis::RightStickX);
			let cy = gamepad.value(gilrs::Axis::RightStickY);
			if cx != 0.0 || cy != 0.0 {
				let cx = (apply_deadzone_min(cx, CURSOR_DEADZONE) * 1.2 * JOYSTICK_DISTANCE as f32) as i32;
				let cy = (apply_deadzone_min(cy, CURSOR_DEADZONE) * 1.2 * JOYSTICK_DISTANCE as f32) as i32;
				self.cursor = GameVec::new(
					cx,
					cy,
				);
				self.cursor = self.cursor.length_clamped(JOYSTICK_DISTANCE);
			}

			self.attack1 |= gamepad.is_pressed(gilrs::Button::RightTrigger2);
			self.attack2 |= gamepad.is_pressed(gilrs::Button::RightTrigger);
			self.special1 |= gamepad.is_pressed(gilrs::Button::LeftTrigger2);
			self.special2 |= gamepad.is_pressed(gilrs::Button::LeftTrigger);
		}

		if has_keyboard {
			let new_mouse_pos = get_mouse_position();
			let mouse_diff = new_mouse_pos - self.mouse_position;
			self.cursor += GameVec::new(mouse_diff.x, -mouse_diff.y) * 6;
			self.cursor = self.cursor.length_clamped(JOYSTICK_DISTANCE);
			self.mouse_position = new_mouse_pos;
		}
	}
}
