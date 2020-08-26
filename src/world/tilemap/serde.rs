use crate::prelude::*;

impl Serialize for TileMap {
	fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where S: Serializer {
		(self.tiles.clone(), self.size).serialize(serializer)
	}
}

impl<'de> Deserialize<'de> for TileMap {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
	{
		<(Vec<Tile>, TileVec)>::deserialize(deserializer)
			.map(deser)
	}
}

fn deser((tiles, size): (Vec<Tile>, TileVec)) -> TileMap {
	let texture = TileMap::create_texture(&tiles, size);
	TileMap {
		tiles,
		size,
		texture
	}
}