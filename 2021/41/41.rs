struct Counter {
	n: i32,
	n_end: i32,
}

impl Counter {
	fn new(start: i32, end: i32) -> Self {
		Counter { n: start, n_end: end }
	}
}

impl Iterator for Counter {
	type Item = i32;
	fn next(&mut self) -> Option<Self::Item> {
        let ret = self.n;
        self.n += 1;
        if ret <= self.n_end {
            Some(ret)
        } else {
            None
        }
	}
}

fn main() {
	println!("{:?}", Counter::new(3, 10).collect::<Vec<_>>());
}
