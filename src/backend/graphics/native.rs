use crate::prelude::*;

pub struct NativeGraphicsBackend {
	pub draw_sender: Sender<Draw>,
}

impl GraphicsBackend for NativeGraphicsBackend {
	fn draw(&mut self, draw: Draw) {
		self.draw_sender.send(draw).unwrap();
	}
}
