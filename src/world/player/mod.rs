mod render;

use sfml::system::Vector2f;

pub struct Player {
    // position is the center of the player
    pub position: Vector2f,
    // size.x is the half width of the player and size.y is the half height of the player
    pub size: Vector2f,
    pub speed: Vector2f,
    pub direction: Vector2f,
}

impl Player {
    pub fn new(position: Vector2f) -> Player {
        Player {
            position,
            size: Player::get_size(),
            speed: Vector2f::new(0.0, 0.0),
            direction: Vector2f::new(0.0, 0.0),
        }
    }

    pub fn get_size() -> Vector2f {
        Vector2f::new(100.0, 160.0)
    }

    pub fn tick(&mut self) {
        self.speed += self.direction;
        self.position += self.speed;
    }
}
