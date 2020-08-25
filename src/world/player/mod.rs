mod draw;
mod physics;
pub mod sensor;

use crate::prelude::*;

// The left-bottom of the tile (x,y) is located at position (x,y) * TILESIZE.
pub const PLAYER_SIZE: GameVec = TileVec::new(2, 6).to_game();
pub const MAX_HEALTH: i32 = 100;

const X_DRAG: i32 = 30;
const MAX_X_VEL: i32 = 120;
const JUMP_POWER: i32 = 300;
const X_ACCELERATION: i32 = 55;
pub const CURSOR_INDICATOR_RADIUS: i32 = TILESIZE / 2;

// also required for fluids!
pub const GRAVITY: i32 = 15;

static GROUND_SENSOR: Sensor = Sensor {
	left_bot_offset: GameVec::new(0, -1),
	size: GameVec::new(PLAYER_SIZE.x, 2),
};

static LEFT_SENSOR: Sensor = Sensor {
	left_bot_offset: GameVec::new(-TILESIZE/2, PLAYER_SIZE.y / 4),
	size: GameVec::new(0, PLAYER_SIZE.y * 3 / 4),
};

static RIGHT_SENSOR: Sensor = Sensor {
	left_bot_offset: GameVec::new(PLAYER_SIZE.x, PLAYER_SIZE.y / 4),
	size: GameVec::new(PLAYER_SIZE.x + TILESIZE/2, PLAYER_SIZE.y * 3 / 4),
};

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum PlayerDirection {
	Left,
	Right,
}
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum PlayerColor {
	Blue,
	Red,
}

pub struct Player {
	pub left_bot: GameVec,
	pub velocity: GameVec,
	pub animation: Animation,
	pub cursor: GameVec,
	pub health: i32,
	pub free_wall: u32,
	pub last_wall_pos: Option<GameVec>,
	walljumped: bool,
	direction: PlayerDirection,
	color: PlayerColor,
}

impl Player {
	pub fn new(left_bot: GameVec, direction: PlayerDirection, color: PlayerColor) -> Player {
		Player {
			left_bot,
			velocity: GameVec::new(0, 0),
			animation: Animation::new(AnimationId::BluePlayerIdle),
			cursor: GameVec::new(0, 0),
			health: MAX_HEALTH,
			free_wall: 0,
			last_wall_pos: None,
			walljumped: true,
			direction,
			color,
		}
	}

	pub fn tick(&mut self, t: &mut TileMap, input: &dyn Input) {
		self.select_animation(t);
		self.apply_forces(input, t);
		self.move_by_velocity(t);
	}

	fn select_animation(&mut self, t: &TileMap) {
		let (run, idle, jump, fall, fall_slow) = if self.color == PlayerColor::Blue {
			(AnimationId::BluePlayerRun, AnimationId::BluePlayerIdle, AnimationId::BluePlayerJump, AnimationId::BluePlayerFall, AnimationId::BluePlayerFallSlow)
		} else {
			(AnimationId::RedPlayerRun, AnimationId::RedPlayerIdle, AnimationId::RedPlayerJump, AnimationId::RedPlayerFall, AnimationId::RedPlayerFallSlow)
		};

		let new_animation_id = if self.is_grounded(t) {
			if self.velocity.x.abs() > 10 {
				run
			} else {
				idle
			}
		} else {
			if self.velocity.y > 70 {
				jump
			} else if self.velocity.y > -70 {
				fall_slow
			} else {
				fall
			}
		};

		if new_animation_id != self.animation.animation_id {
			self.animation = Animation::new(new_animation_id);
		}

		self.direction = if self.velocity.x < 0 {
			PlayerDirection::Left
		} else if self.velocity.x > 0 {
			PlayerDirection::Right
		} else {
			self.direction
		}
	}

	fn apply_forces(&mut self, input: &dyn Input, t: &TileMap) {
		// drag
		if self.velocity.x.abs() < X_DRAG { self.velocity.x = 0; }
		else { self.velocity.x -= X_DRAG * self.velocity.x.signum(); }

		// walk
		self.velocity.x += input.horizontal_dir() * X_ACCELERATION;
		if self.velocity.x.abs() > MAX_X_VEL { self.velocity.x = MAX_X_VEL * self.velocity.x.signum(); }

		// jump
		if self.is_grounded(t) && input.up() && self.velocity.y <= 0 {
			self.velocity.y = JUMP_POWER;
			self.walljumped = false;
		}

		// walljump
		if !self.walljumped && !self.is_grounded(t) && input.up() && (
				self.is_left_walled(t) && input.right() ||
				self.is_right_walled(t) && input.left()) {
			let horizontal_dir = i32::signum(input.horizontal_dir()) * 100;
			let force = GameVec::new(horizontal_dir, JUMP_POWER);
			self.velocity = force;
			self.walljumped = true;
		}

		// gravity
		self.velocity.y -= GRAVITY;

		// aim
		self.cursor = input.cursor();
	}

	fn is_grounded(&self, t: &TileMap) -> bool {
		self.check_sensor(&GROUND_SENSOR, t)
	}

	fn is_left_walled(&self, t: &TileMap) -> bool {
		self.check_sensor(&LEFT_SENSOR, t)
	}

	fn is_right_walled(&self, t: &TileMap) -> bool {
		self.check_sensor(&RIGHT_SENSOR, t)
	}

	pub fn center_position(&self) -> GameVec {
		self.left_bot + PLAYER_SIZE / 2
	}

	pub fn cursor_position(&self) -> GameVec {
		self.center_position() + self.cursor
	}

	pub fn damage(&mut self, dmg: i32) {
		self.health -= dmg;
		self.health = i32::max(0, self.health);
	}

	pub fn collides_point(&self, p: GameVec) -> bool {
		self.left_bot.x <= p.x && p.x <= self.left_bot.x + PLAYER_SIZE.x - 1
			&& self.left_bot.y <= p.y && p.y <= self.left_bot.y + PLAYER_SIZE.y - 1
	}

	pub fn collides_tile(&self, t: TileVec) -> bool {
		let t_lb = t.to_game();
		let t_rt = (t + (1,1)).to_game() - (1,1);

		let p_lb = self.left_bot;
		let p_rt = self.left_bot + PLAYER_SIZE - (1,1);

		t_lb.x <= p_rt.x &&
		t_lb.y <= p_rt.y &&
		p_lb.x <= t_rt.x &&
		p_lb.y <= t_rt.y
	}
}
