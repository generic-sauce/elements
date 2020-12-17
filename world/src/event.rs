pub trait EventHandler {
	fn tilemap_changed(&mut self);
	fn damage_inflicted(&mut self, damage: i32, player: usize);
}

impl EventHandler for () {
	fn tilemap_changed(&mut self) {}
	fn damage_inflicted(&mut self, _damage: i32, _player: usize) {}
}
