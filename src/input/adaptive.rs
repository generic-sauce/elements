use crate::prelude::*;

use super::*;

pub struct AdaptiveInput {
	direction: GameVec,
	just_up: bool,
	just_down: bool,
	special1: bool,
	special2: bool,
	attack1: bool,
	attack2: bool,
	just_attack2: bool,
	cursor: GameVec,
	gamepad_id: Option<GamepadId>,
	has_keyboard: bool,
}

impl InputDevice {
	pub fn new_adaptive(index: u32, has_keyboard: bool, gilrs: &gilrs::Gilrs) -> InputDevice {
		let adaptive_input = AdaptiveInput {
			direction: GameVec::new(0, 0),
			just_up: false,
			just_down: false,
			special1: false,
			special2: false,
			attack1: false,
			attack2: false,
			just_attack2: false,
			cursor: GameVec::new(0, 0),
			gamepad_id: get_gamepad(index, gilrs),
			has_keyboard
		};

		InputDevice::Adaptive(adaptive_input)
	}
}

impl AdaptiveInput {
	fn up(&self) -> bool { self.vertical_dir() >= (DEADZONE_MIN * MAX_MOVEMENT_VALUE as f32) as i32 }
	fn down(&self) -> bool { self.vertical_dir() <= (-DEADZONE_MIN * MAX_MOVEMENT_VALUE as f32) as i32 }

	#[allow(unused)]
	fn right(&self) -> bool { self.horizontal_dir() >= (DEADZONE_MIN * MAX_MOVEMENT_VALUE as f32) as i32 }
	#[allow(unused)]
	fn left(&self) -> bool { self.horizontal_dir() <= (-DEADZONE_MIN * MAX_MOVEMENT_VALUE as f32) as i32 }

	fn horizontal_dir(&self) -> i32 { self.direction.x }
	fn vertical_dir(&self) -> i32 { self.direction.y }

	pub fn update(&mut self, gilrs: &gilrs::Gilrs) -> InputState {
		let gamepad = self.gamepad_id.map(|x| gilrs.gamepad(x));

		use sfml::window::{Key::*, mouse::Button};

		let last_frame_up = self.up();
		let last_frame_down = self.down();
		let last_frame_attack2 = self.attack2;

		let mut direction = GameVec::new(0, 0);

		direction.y += if self.has_keyboard && W.is_pressed() {
			MAX_MOVEMENT_VALUE
		} else if self.has_keyboard && S.is_pressed() {
			-MAX_MOVEMENT_VALUE
		} else if let Some(gamepad) = gamepad {
			(apply_deadzone(gamepad.value(gilrs::Axis::LeftStickY)) * MAX_MOVEMENT_VALUE as f32) as i32
		} else {
			0
		};

		let mut key_pressed = false;
		if self.has_keyboard && D.is_pressed() {
			direction.x += MAX_MOVEMENT_VALUE;
			key_pressed = true;
		}
		if self.has_keyboard && A.is_pressed(){
			direction.x -= MAX_MOVEMENT_VALUE;
			key_pressed = true;
		}
		if !key_pressed {
			if let Some(gamepad) = gamepad {
				direction.x = (apply_deadzone(gamepad.value(gilrs::Axis::LeftStickX)) * MAX_MOVEMENT_VALUE as f32) as i32
			}
		}

		self.attack1 = self.has_keyboard && (Q.is_pressed() || Button::Left.is_pressed());
		self.attack2 = self.has_keyboard && E.is_pressed();
		self.special1 = self.has_keyboard && (R.is_pressed() || Button::Right.is_pressed());
		self.special2 = self.has_keyboard && F.is_pressed();

		self.just_up = !last_frame_up && self.up();
		self.just_down = !last_frame_down && self.down();
		self.just_attack2 = !last_frame_attack2 && self.attack2;

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

		if self.has_keyboard {
			let new_mouse_pos = get_mouse_position();
			let mouse_diff = new_mouse_pos - DEFAULT_MOUSE_POSITION;

			sfml::window::mouse::set_desktop_position(DEFAULT_MOUSE_POSITION.to_vector2i());
			let mouse_diff_scaled = mouse_diff * 9.0;
			self.cursor += GameVec::new(mouse_diff_scaled.x as i32, -mouse_diff_scaled.y as i32);
			self.cursor = self.cursor.length_clamped(JOYSTICK_DISTANCE);
		}

		InputState {
			direction,
			cursor: self.cursor,
			just_up: self.just_up,
			just_down: self.just_down,
			special1: self.special1,
			special2: self.special2,
			attack1: self.attack1,
			attack2: self.attack2,
			just_attack2: self.just_attack2,
		}
	}
}

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

fn get_gamepad(index: u32, gilrs: &gilrs::Gilrs) -> Option<GamepadId> {
	gilrs.gamepads()
		.map(|(gamepad_id, _)| gamepad_id)
		.find(|gamepad_id| Into::<usize>::into(*gamepad_id) == index as usize)
}

fn get_mouse_position() -> WindowVec {
	let mp = sfml::window::mouse::desktop_position();
	WindowVec::new(mp.x as f32, mp.y as f32)
}
