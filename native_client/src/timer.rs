use crate::prelude::*;

#[derive(Copy, Clone)]
pub struct Timer {
    start_time: f64,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            start_time: NativeBackend::now(),
        }
    }

    pub fn elapsed_ms(self) -> f32 {
        (NativeBackend::now() - self.start_time) as f32
    }
}
