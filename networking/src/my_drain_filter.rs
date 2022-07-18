pub struct MyDrainFilterIter<T>(Vec<T>);

impl<T> Iterator for MyDrainFilterIter<T> {
	type Item = T;

	fn next(&mut self) -> Option<T> {
		self.0.pop()
	}
}

pub trait MyDrainFilter<T> {
	fn my_drain_filter(&mut self, f: impl Fn(&T) -> bool) -> MyDrainFilterIter<T>;
}

impl<T> MyDrainFilter<T> for Vec<T> {
	fn my_drain_filter(&mut self, f: impl Fn(&T) -> bool) -> MyDrainFilterIter<T> {
		let mut out = Vec::new();
		while let Some(i) = self.iter().position(&f) {
			out.push(self.remove(i));
		}
		out.reverse();

		MyDrainFilterIter(out)
	}
}
