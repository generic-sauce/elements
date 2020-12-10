pub fn rng(rate: u32, antirate: u32, _seed: [u32; 4]) -> bool {
	let result = rand::random::<u32>();
	result % (rate + antirate) < rate
}
