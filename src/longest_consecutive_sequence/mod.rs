use crate::base::Solution;
use std::collections::HashMap;

impl Solution {
	pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
		let mut map: HashMap<i32, usize> = HashMap::new();
		for item in nums.iter() {
			map.insert(*item, 1);
		}
		let mut res = 0;
		unsafe {
			for item in map.iter() {
				let mut i = *item.0;
				if *item.1 == 0 {
					continue;
				}
				let t = std::mem::transmute::<*const _, *mut usize>(item.1);
				while let Some(v) = map.get(&(i + 1)) {
					let v = std::mem::transmute::<*const _, *mut usize>(v);
					if *v == 1 {
						*t += 1;
						*v = 0;
						i += 1;
					} else {
						*t += *v;
						*v = 0;
						break;
					}
				}
				res = std::cmp::max(res, *t);
			}
		}
		res as i32
	}
}

#[test]
fn test() {
	let x = Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]);
	assert_eq!(x, 4);
}