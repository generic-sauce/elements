use crate::prelude::*;

pub struct WindowParam;
pub type WindowVec = Vec2t<f32, WindowParam>;

#[cfg(feature = "native-client")]
mod only_sfml {
    use crate::prelude::*;

    pub trait ToWindowVec {
        fn to_window(self) -> WindowVec;
    }

    pub trait ToSfmlVec {
        fn to_vector2f(self) -> Vector2f;
        fn to_vector2i(self) -> Vector2i;
        fn to_vector2u(self) -> Vector2u;
    }

    impl ToWindowVec for Vector2f { fn to_window(self) -> WindowVec { WindowVec::new(self.x, self.y) } }

    impl ToWindowVec for Vector2i { fn to_window(self) -> WindowVec { WindowVec::new(self.x as f32, self.y as f32) } }

    impl ToWindowVec for Vector2u { fn to_window(self) -> WindowVec { WindowVec::new(self.x as f32, self.y as f32) } }

    impl ToSfmlVec for WindowVec {
        fn to_vector2f(self) -> Vector2f { Vector2f::new(self.x, self.y) }
        fn to_vector2i(self) -> Vector2i { Vector2i::new(self.x as i32, self.y as i32) }
        fn to_vector2u(self) -> Vector2u {
            assert!(self.x >= 0.0 && self.y >= 0.0);
            Vector2u::new(self.x as u32, self.y as u32)
        }
    }
}