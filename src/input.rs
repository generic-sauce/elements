use crate::prelude::*;
use gilrs::GamepadId;

const CONTROLLER_MAX: i32 = 100;
const DEADZONE_MIN: i32 = 35;
const DEADZONE_MAX: i32 = CONTROLLER_MAX - DEADZONE_MIN;
const JOYSTICK_DISTANCE: i32 = 1500;

fn apply_deadzone(value: i32) -> i32 {
	let sign = value.signum();
	if value.abs() < DEADZONE_MIN {
		0
	} else if value.abs() > DEADZONE_MAX {
		sign * CONTROLLER_MAX
	} else {
		(value.abs()*CONTROLLER_MAX / (DEADZONE_MAX-DEADZONE_MIN) - ((CONTROLLER_MAX * DEADZONE_MIN) / (DEADZONE_MAX-DEADZONE_MIN))) * sign
	}
}

pub trait Input {
	fn horizontal_dir(&self) -> i32;
	fn vertical_dir(&self) -> i32;

	fn up(&self) -> bool { self.vertical_dir() >= DEADZONE_MIN }
	fn just_up(&self) -> bool;
	fn down(&self) -> bool { self.vertical_dir() <= -DEADZONE_MIN }
	fn just_down(&self) -> bool;
	fn right(&self) -> bool { self.horizontal_dir() >= DEADZONE_MIN }
	fn left(&self) -> bool { self.horizontal_dir() <= -DEADZONE_MIN }
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
}

fn get_gamepad(index: u32, gilrs: &gilrs::Gilrs) -> Option<GamepadId> {
	gilrs.gamepads()
		.map(|(gamepad_id, _)| gamepad_id)
		.find(|gamepad_id| Into::<usize>::into(*gamepad_id) == index as usize)
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

		let up_key = if self.index == 0 { sfml::window::Key::W } else { sfml::window::Key::Up };
		let down_key = if self.index == 0 { sfml::window::Key::S } else { sfml::window::Key::Down };
		let right_key = if self.index == 0 { sfml::window::Key::D } else { sfml::window::Key::Right };
		let left_key = if self.index == 0 { sfml::window::Key::A } else { sfml::window::Key::Left };

		let last_frame_up = self.up();
		let last_frame_down = self.down();

		self.direction.x = 0;
		self.direction.y = 0;

		self.direction.y += if up_key.is_pressed() {
			CONTROLLER_MAX
		} else if down_key.is_pressed() {
			-CONTROLLER_MAX
		} else if let Some(gamepad) = gamepad {
			apply_deadzone((gamepad.value(gilrs::Axis::LeftStickY) * 100.0) as i32)
		} else {
			0
		};

		let mut key_pressed = false;
		if right_key.is_pressed() {
			self.direction.x += CONTROLLER_MAX;
			key_pressed = true;
		}
		if left_key.is_pressed() {
			self.direction.x -= CONTROLLER_MAX;
			key_pressed = true;
		}
		if !key_pressed {
			if let Some(gamepad) = gamepad {
				self.direction.x = apply_deadzone((gamepad.value(gilrs::Axis::LeftStickX) * 100.0) as i32)
			}
		}

		self.just_up = !last_frame_up && self.up();
		self.just_down = !last_frame_down && self.down();

		if let Some(gamepad) = gamepad {
			self.cursor = GameVec::new(
				(gamepad.value(gilrs::Axis::RightStickX) * 2000.0) as i32,
				(gamepad.value(gilrs::Axis::RightStickY) * 2000.0) as i32,
			);
			self.cursor = self.cursor.length_clamped(JOYSTICK_DISTANCE);
		}
	}
}
