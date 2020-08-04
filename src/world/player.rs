use sfml::system::Vector2f;

const DEFAULT_PLAYER_SIZE: Vector2f = Vector2f::new(10.0, 20.0);

struct Player {
    // position is the center of the player
    pub position: Vector2f,
    // size.x is the half width of the player and size.y is the half height of the player
    pub size: Vector2f,
}

impl Player {
    fn new(position: Vector2f) -> Player {
        return Player {
            position,
            size: DEFAULT_PLAYER_SIZE,
        }
    }
}