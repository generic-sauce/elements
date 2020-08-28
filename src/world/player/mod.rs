mod draw;
mod physics;
pub mod sensor;

use crate::prelude::*;

// The left-bottom of the tile (x,y) is located at position (x,y) * TILESIZE.
pub const PLAYER_SIZE: GameVec = TileVec::new(2, 6).to_game();
pub const MAX_HEALTH: i32 = 1000;

const X_DRAG: i32 = 30;
const MAX_X_VEL: i32 = 120;
const JUMP_POWER: i32 = 300;
const X_ACCELERATION: i32 = 55;
pub const CURSOR_INDICATOR_RADIUS: i32 = TILESIZE / 2;

// also required for fluids!
pub const GRAVITY: i32 = 15;

static GROUND_SENSOR: Sensor = Sensor {
	left_bot_offset: GameVec::new(0, -TILESIZE-1),
	size: GameVec::new(PLAYER_SIZE.x, TILESIZE+1),
};

static LEFT_SENSOR: Sensor = Sensor {
	left_bot_offset: GameVec::new(-TILESIZE/2, PLAYER_SIZE.y / 4),
	size: GameVec::new(0, PLAYER_SIZE.y * 3 / 4),
};

static RIGHT_SENSOR: Sensor = Sensor {
	left_bot_offset: GameVec::new(PLAYER_SIZE.x, PLAYER_SIZE.y / 4),
	size: GameVec::new(PLAYER_SIZE.x + TILESIZE/2, PLAYER_SIZE.y * 3 / 4),
};

#[derive(PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub enum PlayerDirection {
	Left,
	Right,
}
#[derive(PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub enum PlayerColor {
	Blue,
	Red,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
	pub left_bot: GameVec,
	pub velocity: GameVec,
	pub animation: Animation,
	pub cursor: GameVec,
	pub health: i32,
	pub free_wall: u32,
	pub last_wall_pos: Option<GameVec>,
	pub grab_cooldown: Option<u32>,
	walljumped: bool,
	direction: PlayerDirection,
	color: PlayerColor,
}

impl World {
	pub fn tick_player(&mut self, p: usize, input: &InputState) {
		let pl = &mut self.players[p];
		pl.select_animation(&self.tilemap);
		pl.apply_forces(input, &self.tilemap);
		pl.move_by_velocity(&self.tilemap);

		pl.grab_cooldown = match pl.grab_cooldown {
			None => None,
			Some(0) => None,
			Some(x) => Some(x-1),
		};
	}
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
			grab_cooldown: None,
			walljumped: true,
			direction,
			color,
		}
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

	fn apply_forces(&mut self, input: &InputState, t: &TileMap) {
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
		let largest = (t.size + 1).to_game() - 1; // TODO correct?
		let ctr = self.center_position();
		let mut global_cursor = ctr + input.cursor;
		global_cursor.x = global_cursor.x.max(0).min(largest.x);
		global_cursor.y = global_cursor.y.max(0).min(largest.y);
		self.cursor.x = global_cursor.x - ctr.x;
		self.cursor.y = global_cursor.y - ctr.y;
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

	pub fn collides_point_with_radius(&self, point: GameVec, radius: i32) -> bool {
		let center = self.center_position();

		let circle_dist = GameVec::new((center.x - point.x).abs(), (center.y - point.y).abs());

		if circle_dist.x > PLAYER_SIZE.x / 2 + radius { return false; }
		if circle_dist.y > PLAYER_SIZE.y / 2 + radius { return false; }

		if circle_dist.x <= PLAYER_SIZE.x / 2 { return true; }
		if circle_dist.y <= PLAYER_SIZE.y / 2 { return true; }

		let sq = |a| a * a;
		let cornerdist_sq = sq(circle_dist.x - PLAYER_SIZE.x / 2) + sq(circle_dist.y - PLAYER_SIZE.y / 2);

		cornerdist_sq <= sq(radius)
	}

	pub fn collides_rect(&self, o_lb: GameVec, o_rt : GameVec) -> bool {
		let p_lb = self.left_bot;
		let p_rt = self.left_bot + PLAYER_SIZE - (1,1);

		o_lb.x <= p_rt.x &&
		o_lb.y <= p_rt.y &&
		p_lb.x <= o_rt.x &&
		p_lb.y <= o_rt.y
	}

	pub fn collides_tile(&self, t: TileVec) -> bool {
		let o_lb = t.to_game();
		let o_rt = (t + 1).to_game() - 1;

		self.collides_rect(o_lb, o_rt)
	}
}
