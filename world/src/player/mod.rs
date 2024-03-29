mod input_state;
mod physics;

mod sensor;
pub use sensor::*;

pub use input_state::*;

use crate::prelude::*;

// The left-bottom of the tile (x,y) is located at position (x,y) * TILESIZE.
const PLAYER_SCALE: f32 = 0.93;
pub const PLAYER_SIZE: GameVec = GameVec::new((2.0*TILESIZE as f32 * PLAYER_SCALE) as i32, (6.0*TILESIZE as f32 * PLAYER_SCALE) as i32);
pub const MAX_HEALTH: i32 = 1000;

const X_DRAG: i32 = 30;
const MAX_X_VEL: i32 = 120;
const JUMP_POWER: i32 = 300;
const X_ACCELERATION: i32 = 55;
const JOYSTICK_DISTANCE: i32 = 2600;
const MAX_MOVEMENT_VALUE: i32 = 100;

// also required for fluids!
pub const GRAVITY: i32 = 15;

static GROUND_SENSOR: Sensor = Sensor {
	left_bot_offset: GameVec::new(0, -TILESIZE-1),
	size: GameVec::new(PLAYER_SIZE.x, TILESIZE+1),
};

const WALL_JUMP_SENSOR_WIDTH: i32 = TILESIZE;
static LEFT_SENSOR: Sensor = Sensor {
	left_bot_offset: v(-WALL_JUMP_SENSOR_WIDTH, 0),
	size: v(WALL_JUMP_SENSOR_WIDTH, PLAYER_SIZE.y * 3 / 4),
};

static RIGHT_SENSOR: Sensor = Sensor {
	left_bot_offset: v(PLAYER_SIZE.x, 0),
	size: v(WALL_JUMP_SENSOR_WIDTH, PLAYER_SIZE.y * 3 / 4),
};

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum PlayerDirection {
	Left,
	Right,
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum XDir {
	Left, Right
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum WallMode {
	NoFluids, // when you are InProgress, and then the fluids run out.
	InProgress { last_drawn_tile: TileVec }, // while you are drawing the wall.
	NotWalling, // when you are not drawing the wall.
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HealthBarStatus {
	pub health_delayed: i32,
	pub health_delayed_counter: i32,
	pub red_death_counter: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
	pub left_bot: GameVec,
	pub velocity: GameVec,
	pub cursor: GameVec,
	pub health: i32,
	pub health_bar_status: HealthBarStatus, // used to draw healthbar
	pub wall_mode: WallMode,
	pub free_wall_lifetime: u32,
	pub grab_cooldown: Option<u32>,
	pub animation: Animation,
	pub direction: PlayerDirection,
	pub last_walljump: Option<XDir>, // XDir::Left means the last walljump happened to a wall on a wall left of the player
	pub input: InputState,
}

const HEALTH_DELAY_SPEED: i32 = 15;
const HEALTH_DELAY: i32 = 6;
pub const RED_HEALTH_DEATH_DURATION: i32 = 25;

impl HealthBarStatus {
	pub fn tick(&mut self, health: i32) {
		if health <= 0 {
			self.health_delayed = 0;
			self.red_death_counter -= 1;
			return;
		} else {
			self.red_death_counter = RED_HEALTH_DEATH_DURATION;
		}
		if self.health_delayed_counter <= 0 {
			if health < self.health_delayed {
				self.health_delayed = health.max(self.health_delayed - HEALTH_DELAY_SPEED); // make sure health_delayed is not smaller than health
				if self.health_delayed == health {
					self.health_delayed_counter = HEALTH_DELAY;
				}
			}
		} else {
			if health < self.health_delayed {
				self.health_delayed_counter -= 1;
			}
		}
	}
}

impl World {
	pub fn tick_player(&mut self, p: usize) {
		let pl = &mut self.players[p];
		pl.animation.tick();
		pl.select_animation(self.teams[p], &self.tilemap);
		pl.apply_forces(&self.tilemap);
		pl.move_by_velocity(&self.tilemap);
		pl.health_bar_status.tick(pl.health);

		pl.grab_cooldown = match pl.grab_cooldown {
			None => None,
			Some(0) => None,
			Some(x) => Some(x-1),
		};
	}
}

impl Player {
	pub fn new(left_bot: GameVec, animation_id: AnimationId, direction: PlayerDirection) -> Player {
		Player {
			left_bot,
			velocity: GameVec::new(0, 0),
			cursor: GameVec::new(0, 0),
			health: MAX_HEALTH,
			health_bar_status: HealthBarStatus {
				health_delayed_counter: HEALTH_DELAY,
				health_delayed: MAX_HEALTH,
				red_death_counter: RED_HEALTH_DEATH_DURATION,
			},
			wall_mode: WallMode::NotWalling,
			free_wall_lifetime: 0,
			grab_cooldown: None,
			animation: Animation::new(animation_id),
			direction,
			last_walljump: None,
			input: InputState::new(),
		}
	}

	fn select_animation(&mut self, team: u8, t: &TileMap) {
		let (run, idle, jump, fall, fall_slow) = if team == 0 {
			(AnimationId::BluePlayerRun, AnimationId::BluePlayerIdle, AnimationId::BluePlayerJump, AnimationId::BluePlayerFall, AnimationId::BluePlayerFallSlow)
		} else if team == 1 {
			(AnimationId::RedPlayerRun, AnimationId::RedPlayerIdle, AnimationId::RedPlayerJump, AnimationId::RedPlayerFall, AnimationId::RedPlayerFallSlow)
		} else {
			panic!("select_animation team out of range!");
		};

		let animation_id = if self.is_grounded(t) {
			if self.velocity.x.abs() > 10 {
				run
			} else {
				idle
			}
		} else if self.velocity.y > 70 {
			jump
		} else if self.velocity.y > -70 {
			fall_slow
		} else {
			fall
		};

		self.direction = match self.velocity.x.cmp(&0) {
			Ordering::Less => PlayerDirection::Left,
			Ordering::Equal => self.direction,
			Ordering::Greater => PlayerDirection::Right,
		};

		if self.animation.animation_id != animation_id {
			self.animation = Animation::new(animation_id);
		}
	}

	fn apply_forces(&mut self, t: &TileMap) {
		// drag
		if self.velocity.x.abs() < X_DRAG { self.velocity.x = 0; }
		else { self.velocity.x -= X_DRAG * self.velocity.x.signum(); }

		// walk
		self.velocity.x += (self.input.horizontal_dir() * X_ACCELERATION as f32 * MAX_MOVEMENT_VALUE as f32) as i32;
		if self.velocity.x.abs() > MAX_X_VEL { self.velocity.x = MAX_X_VEL * self.velocity.x.signum(); }

		// jump
		if self.is_grounded(t) && self.input.up() && self.velocity.y <= 0 {
			self.velocity.y = JUMP_POWER;
			self.last_walljump = None;
		}

		// walljump
		if !self.is_grounded(t) && self.input.up() {
			let arr = [
				Some(XDir::Left).filter(|_| self.is_left_walled(t) && self.input.right() && self.last_walljump != Some(XDir::Left)),
				Some(XDir::Right).filter(|_| self.is_right_walled(t) && self.input.left() && self.last_walljump != Some(XDir::Right)),
			];
			let mut iter = arr.iter().filter_map(|x| *x);

			if let Some(dir) = iter.next() {
				let horizontal_dir = match dir {
					XDir::Left => -1,
					XDir::Right => 1,
				} * -100;
				self.last_walljump = Some(dir);
				self.velocity = v(horizontal_dir, JUMP_POWER);
			}
		}

		// gravity
		self.velocity.y -= GRAVITY;

		// aim
		let largest = (t.size + 1).to_game() - 1; // TODO correct?
		let ctr = self.center_position();
		let cursor = GameVec::new((self.input.cursor.x * JOYSTICK_DISTANCE as f32) as i32, (self.input.cursor.y * JOYSTICK_DISTANCE as f32) as i32);
		let mut global_cursor = ctr + cursor;
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

	pub fn collides_fluid(&self, point: GameVec, radius: i32, t: &TileMap) -> bool {
		// this is the closest point within the player to `point`
		let closest: GameVec = v(
			point.x.min(self.left_bot.x + PLAYER_SIZE.x).max(self.left_bot.x),
			point.y.min(self.left_bot.y + PLAYER_SIZE.y).max(self.left_bot.y),
		);

		if !(closest - point).as_short_as(radius) { return false; }

		// raycast in order to not damage through walls
		if t.check_solid(point) { return false; }

		let mut last_p = point;
		let n = radius;

		for i in 0..=n {
			let p = (point * (n-i)) / n + (closest * i) / n;
			// if they go diagonal
			if !(p.to_tile() - last_p.to_tile()).as_short_as(1) {
				let between: TileVec = v(p.to_tile().x, last_p.to_tile().y);
				if t.check_solid(between) { return false; }
			}
			if t.check_solid(p) { return false; }

			last_p = p;
		}

		true
	}
}
