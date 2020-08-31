use crate::prelude::*;

impl<T, P> Serialize for Vec2t<T, P> where T: Serialize + Copy {
	fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where S: Serializer {
		(self.x, self.y).serialize(serializer)
	}
}
impl<'de, T, P> Deserialize<'de> for Vec2t<T, P> where T: Deserialize<'de> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
	{
		<(T, T)>::deserialize(deserializer)
			.map(Into::into)
	}
}