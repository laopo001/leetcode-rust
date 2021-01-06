struct Solution;
use std::collections::HashSet;

fn run(x: i32, y: i32, a: u32, b: u32, bound: i32, arr: &mut HashSet<i32>) {
	let res = x.pow(a) + y.pow(b);
	if res <= bound {
		arr.insert(res);
		if x != 1 {
			run(x, y, a + 1, b, bound, arr);
		}
		if y != 1 {
			run(x, y, a, b + 1, bound, arr);
		}
	}
}

impl Solution {
	// 0 ms, faster than 100.00%
	pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
		let mut res: HashSet<i32> = HashSet::new();
		run(x, y, 0, 0, bound, &mut res);
		res.iter().cloned().collect()
	}
}