use crate::base::Solution;
use std::collections::HashMap;

impl Solution {
	pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
		let mut map: HashMap<i32, usize> = HashMap::new();
		for item in nums.iter() {
			map.insert(*item, 1);
		}
		let mut res = 0;
		// Repeatedly try this test.
		unsafe {
			for item in map.iter() {
//			map.remove(item.0);
				while let Some(v) = map.get(&(*item.0 + 1)) {
					std::mem::transmute::<&usize, &mut usize>(item.1);
					if *v == 0 {
						*item.1 += 1;
					} else {
						*item.1 += v;
					}
				}
			}
		}
		0
	}
}