struct Solution;

impl Solution {
	pub fn longest_arith_seq_length(a: Vec<i32>) -> i32 {
		use std::collections::{HashMap, HashSet};
		use std::cmp::max;
		// let mut map: HashMap<i32, bool> = HashMap::new();
		// let mut set: HashSet<i32> = a.into_iter().collect();
		// let mut a: Vec<i32> = set.into_iter().collect();
		let len = a.len();
		// a.sort();
		let mut dp: Vec<HashMap<i32, usize>> = vec![HashMap::new(); len];
		let mut res = 0;
		for i in 0..len {
			let mut map: HashMap<i32, i32> = HashMap::new();
			for j in 0..i {

				let t = a[i] - a[j];
				if !dp[i].contains_key(&t) {
					dp[i].insert(t, 1);
				}
				let value = dp[i].get(&t).unwrap();

				unsafe {
					let v = std::mem::transmute::<*const _, *mut usize>(value);
					let value2 = dp[j].get(&t).unwrap_or(&1);
					if map.contains_key(&a[j]) && map.get(&a[j]).unwrap() + a[j] == a[i] {
						continue;
					} else {
						map.insert(a[j], 1);
					}
					// println!("{:?}", *v);
					*v += *value2;
					res = max(res, *v as i32);
				}
			}
		}
		// println!("{:?}", a);
		// println!("{:?}", dp);
		res
	}
}