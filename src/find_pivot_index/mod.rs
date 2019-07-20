use crate::base::Solution;
use std::collections::HashMap;


fn sum_nums(l: usize, r: usize, nums: &Vec<i32>, map: &mut Vec<Vec<Option<i32>>>) -> i32 {
	if map[l][r].is_some() {
		return map[l][r].unwrap();
	}
	if r <= l {
		return 0;
	}
	let mut res: i32;
	if l + 1 == r {
		res = nums[l];
	} else {
		res = nums[l] + sum_nums(l + 1, r, nums, map);
	}
	map[l][r] = Some(res);
	return res;
}

impl Solution {
	// Time Limit Exceeded
	pub fn pivot_index(nums: Vec<i32>) -> i32 {
		let len = nums.len();
		let mut res = -1;
//		let mut map: HashMap<(usize, usize), i32> = HashMap::new();
		let mut map: Vec<Vec<Option<i32>>> = vec![vec![None; len]; len];
		for i in 0..len {
			if sum_nums(0, i, &nums, &mut map) == sum_nums(i + 1, len, &nums, &mut map) {
				res = i as i32;
				break;
			}
		}
		res
	}
}