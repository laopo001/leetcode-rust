use crate::base::Solution;
use std::collections::HashMap;

impl Solution {
	pub fn beautiful_array(n: i32) -> Vec<i32> {
		let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
		return Solution::f(n, &mut map);
	}
	fn f(n: i32, map: &mut HashMap<i32, Vec<i32>>) -> Vec<i32> {
		if map.contains_key(&n) {
			return map.get(&n).unwrap().clone();
		}
		let mut res = vec![0; n as usize];
		if n == 1 {
			res[0] = 1;
		} else {
			let mut i = 0;
			for x in Solution::f((n + 1) / 2, map) {
				res[i] = 2 * x - 1;
				i += 1;
			}
			for x in Solution::f((n) / 2, map) {
				res[i] = 2 * x;
				i += 1;
			}
		}

		let copy = res.clone();
		map.insert(n, res);
		copy
	}
}
