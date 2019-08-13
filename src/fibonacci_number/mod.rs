use crate::base::Solution;
use std::collections::HashMap;

impl Solution {
	pub fn fib(n: i32) -> i32 {
		if n == 0 || n == 1 {
			return n;
		}
//		let mut res = 0;
		let mut map: HashMap<usize, i32> = HashMap::new();
		map.insert(0, 0);
		map.insert(1, 1);
		for i in 2..((n + 1) as usize) {
			map.insert(i, *map.get(&(i - 1)).unwrap() + *map.get(&(i - 2)).unwrap());
		}
		*map.get(&(n as usize)).unwrap()
	}
}