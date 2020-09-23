use crate::prelude::*;

impl World {
	pub(in super) fn handle_brickle(&mut self, p: usize) {
		let player = &self.players[p];
		let iter: Vec<TileVec> = TileMap::iter(self.tilemap.size)
			.filter(|i| (i.to_game() + TILESIZE/2 - player.center_position()).length() < TILESIZE * 5)
			.filter(|i| self.tilemap.get(*i) == Tile::Ground)
			.collect();

		let mut velocity = player.cursor_position();
		velocity = if velocity.x > velocity.y { GameVec::new(i32::signum(velocity.x), 0) } else { GameVec::new(0, i32::signum(velocity.y)) };
		velocity *= 10;

		for i in iter {
			println!("xy");
			self.tilemap.set(i, Tile::Brick(Brick{
				owner: p,
				remaining_lifetime: 0,
				velocity: GameVec::new(-10, 0),
				position: i.into()
			}));
		}
	}
}
