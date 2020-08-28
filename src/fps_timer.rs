#[macro_export]
macro_rules! time {
	($action:expr) => {{
		let before = std::time::SystemTime::now();
		let r = $action;
		let after = std::time::SystemTime::now();

		let time_it_took_micros = after.duration_since(before).unwrap().as_micros();
		let frame_time_micros: u128 = 1000_000/60;
		let ratio = 100 * time_it_took_micros / frame_time_micros;
		println!("{} took {} ms ({}%)", std::stringify!($action), time_it_took_micros as f64 / 1000.0, ratio);
		r
	}}
}

#[macro_export]
macro_rules! time_named {
	($name:expr, $action:expr) => {{
		let before = std::time::SystemTime::now();
		let r = $action;
		let after = std::time::SystemTime::now();

		let time_it_took_micros = after.duration_since(before).unwrap().as_micros();
		let frame_time_micros: u128 = 1000_000/60;
		let ratio = 100 * time_it_took_micros / frame_time_micros;
		println!("{} took {} ms ({}%)", std::stringify!($action), time_it_took_micros as f64 / 1000.0, ratio);
		r
	}}
}
