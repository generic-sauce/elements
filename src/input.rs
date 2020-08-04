use sfml::system::Vector2f;

pub trait Input {
    fn get_direction(&self) -> Vector2f;
}

pub struct KeyboardInput {}

impl KeyboardInput {
    pub fn new() -> KeyboardInput {
        KeyboardInput {}
    }
}

impl Input for KeyboardInput {
    fn get_direction(&self) -> Vector2f {
        let mut direction = Vector2f::new(0.0, 0.0);
        if sfml::window::Key::W.is_pressed() {
            direction.y += 1.0;
        }
        if sfml::window::Key::S.is_pressed() {
            direction.y -= 1.0;
        }

        if sfml::window::Key::D.is_pressed() {
            direction.x += 1.0;
        }
        if sfml::window::Key::A.is_pressed() {
            direction.x -= 1.0;
        }
        direction
    }
}