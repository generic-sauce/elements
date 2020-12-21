pub trait EventHandler {
	fn tilemap_changed(&mut self);
	fn damage_inflicted(&mut self, damage: i32, player: usize);
	fn new_game_started(&mut self);
	fn game_ended(&mut self);
}

impl EventHandler for () {
	fn tilemap_changed(&mut self) {}
	fn damage_inflicted(&mut self, _damage: i32, _player: usize) {}
	fn new_game_started(&mut self) {}
	fn game_ended(&mut self) {}
}
