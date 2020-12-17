use crate::prelude::*;

pub struct AppEventHandler {
    pub tilemap_changed: bool,
    pub damages: [i32; 2],
}

impl AppEventHandler {
    pub fn new() -> AppEventHandler {
        AppEventHandler {
            tilemap_changed: false,
            damages: [0; 2],
        }
    }
}

impl EventHandler for AppEventHandler {
    fn tilemap_changed(&mut self) {
        self.tilemap_changed = true;
    }

    fn damage_inflicted(&mut self, damage: i32, player: usize) {
        self.damages[player] += damage;
    }
}
