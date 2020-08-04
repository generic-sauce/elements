use sfml::system::Vector2f;

pub struct Player {
    // position is the center of the player
    pub position: Vector2f,
    // size.x is the half width of the player and size.y is the half height of the player
    pub size: Vector2f,
}

impl Player {
    pub fn new(position: Vector2f) -> Player {
        return Player {
            position,
            size: Player::get_size(),
        }
    }

    pub fn get_size() -> Vector2f {
        return Vector2f::new(10.0, 20.0);
    }
}