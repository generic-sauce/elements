use crate::prelude::*;

pub struct NativeGraphicsBackend {
	pub draw_sender: Sender<Draw>,
}

impl GraphicsBackend for NativeGraphicsBackend {
	fn draw(&mut self, draw: Draw, _: Option<&World>) {
		self.draw_sender.send(draw).unwrap();
	}
}
