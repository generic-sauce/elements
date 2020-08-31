use crate::prelude::*;

pub enum MusicCommand {
	// TODO
}

pub struct Musician {
	receiver: Receiver<MusicCommand>,
}

impl Musician {
	pub fn new(receiver: Receiver<MusicCommand>) -> Musician {
		Musician { receiver }
	}

	pub fn run(&mut self) {
		for _ in TimedLoop::with_fps(10) {
			match self.receiver.try_recv() {
				Ok(c) => self.apply_command(c),
				Err(TryRecvError::Disconnected) => panic!("musician is disconnected!"),
				Err(TryRecvError::Empty) => {},
			}
		}
	}

	pub fn apply_command(&mut self, c: MusicCommand) {
		match c { }
	}
}
