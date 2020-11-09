use crate::prelude::*;

macro_rules! setup {
	($($id:ident : $resource:expr),*$(,)?) => {

		#[derive(Copy, Clone, Debug)]
		#[repr(usize)]
		#[allow(unused)]
		pub enum TextureId {
			$($id),*
		}

		impl TextureId {
			#[allow(unused)]
			pub fn iter() -> impl Iterator<Item=TextureId> {
				[$(TextureId::$id),*].iter().cloned()
			}

			// TODO use in the web client
			#[allow(unused)]
			pub fn filepath_relative(self) -> &'static str {
				match self {
					$(
						TextureId::$id => $resource,
					)*
				}
			}

			#[allow(unused)]
			pub fn filepath(self) -> String {
				res(self.filepath_relative())
			}
		}
	}
}

setup!(
	Unknown: "images/checkerboard.png",
	White: "images/white.png",
);

impl TextureId {
	pub fn texture_count() -> usize {
		TextureId::iter().count()
		+
		AnimationId::iter()
			.map(AnimationId::frame_count)
			.sum::<usize>()
	}
}

impl IntoTextureIndex for TextureId {
	fn into_texture_index(self) -> usize {
		self as usize
	}
}

impl IntoTextureIndex for Animation {
	fn into_texture_index(self) -> usize {
		let texture_offset = TextureId::iter().count();
		let animation_offset = AnimationId::iter()
			.enumerate()
			.filter(|(index, _)| *index < self.animation_id as usize)
			.map(|(_, id)| id)
			.fold(0, |acc, id| acc + AnimationId::frame_count(id));

		texture_offset + animation_offset + self.texture_index()
	}
}

pub fn texture_filenames_iter() -> impl Iterator<Item=String> {
	TextureId::iter()
		.map(TextureId::filepath)
		// .chain(
		// 	AnimationId::iter()
		// 		.map(AnimationId::dir)
		// 		.flat_map(|dir| std::fs::read_dir(res(&dir)).unwrap())
		// 		.map(|filepath| {
		// 			let filepath = filepath.expect("could not find file");
		// 			let filepath = filepath.path();
		// 			filepath.into_os_string().into_string().expect("could not get filepath")
		// 		})
		// )
}
