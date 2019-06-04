use crate::base::Solution;
use std::collections::HashMap;

impl Solution {
	pub fn majority_element(nums: Vec<i32>) -> i32 {
		let mut map: HashMap<i32, usize> = HashMap::new();
		let len = nums.len();
		for i in nums {
			if map.contains_key(&i) {
				let v = map.get_mut(&i).unwrap();
				*v += 1;
				if *v > len / 2 {
					return i;
				}
			} else {
				map.insert(i, 1);
				if 1 > len / 2 {
					return i;
				}
			}
		}
		return 0;
	}
	pub fn majority_element2(nums: Vec<i32>) -> i32 {
		let mut res = nums[0];
		let mut count = 1;
		for i in 0..nums.len() {
			if nums[i] == res {
				count += 1;
			} else {
				count -= 1;
			}
			if count == 0 {
				res = nums[i];
				count = 1;
			}
		}
		return res;
	}
}