pub fn rng(rate: u32, antirate: u32, seed: [u32; 4]) -> bool {
	let result = mini_rng(mini_rng(seed[0], seed[1]), mini_rng(seed[2], seed[3]));
	result % (rate + antirate) < rate
}

fn mini_rng(a: u32, b: u32) -> u32 {
	use rand::{SeedableRng, RngCore};
	use rand_chacha::ChaCha8Rng;

	let combined = (a as u64) + ((b as u64) << 32);

	ChaCha8Rng::seed_from_u64(combined).next_u32()
}
