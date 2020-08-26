#[macro_export]
macro_rules! time {
	($action:expr) => {{
		let before = std::time::SystemTime::now();
		let r = $action;
		let after = std::time::SystemTime::now();

		let time_it_took_ms = after.duration_since(before).unwrap().as_millis();
		let frame_time_ms: u128 = 1000/60;
		let ratio = 100 * time_it_took_ms / frame_time_ms;
		println!("{} took {} ms ({}%)", std::stringify!($action), time_it_took_ms, ratio);
		r
	}}
}

#[macro_export]
macro_rules! time_named {
	($name:expr, $action:expr) => {{
		let before = std::time::SystemTime::now();
		let r = $action;
		let after = std::time::SystemTime::now();

		let time_it_took_ms = after.duration_since(before).unwrap().as_millis();
		let frame_time_ms: u128 = 1000/60;
		let ratio = 100 * time_it_took_ms / frame_time_ms;
		println!("{:?} took {} ms ({}%)", $name, time_it_took_ms, ratio);
		r
	}}
}
