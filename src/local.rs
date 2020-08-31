use crate::prelude::*;

pub struct Local {
	app: App,
	inputs: [InputDevice; 2],
}

impl Local {
	pub fn new() -> Local {
		let app = App::new();
		let inputs = [InputDevice::new_adaptive(0, false, &app.gilrs), InputDevice::new_adaptive(1, true, &app.gilrs)];

		Local {
			app,
			inputs,
		}
	}

	pub fn run(&mut self) {
		let timed_loop = TimedLoop::with_fps(60);
		let interval = timed_loop.interval;
		for (elapsed_time, delta_time, fps, load) in timed_loop {
			while let Some(event) = self.app.window.poll_event() {
				match event {
					Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => {
						self.app.window.close();
						return;
					}
					_ => {},
				}
			}

			// process gilrs events
			while let Some(_) = self.app.gilrs.next_event() {}

			if delta_time > interval {
				println!("Framedrop. Frame took {}ms instead of {}ms", delta_time.as_millis(), interval.as_millis());
			}

			// inputs
			for (i, input) in self.inputs.iter_mut().enumerate() {
				self.app.world.players[i].input = input.update(&self.app.gilrs);
			}

			self.app.tick();
			self.app.draw(elapsed_time, fps, load);

			self.check_restart();

			if !self.app.window.is_open() {
				break;
			}
		}
	}

	fn check_restart(&mut self) {
		if let Some(p) = self.app.world.player_dead() {
			self.app.world.kills[1-p] += 1;
			let cmds = self.app.world.reset();
			self.app.apply_commands(cmds);
		}
	}
}
