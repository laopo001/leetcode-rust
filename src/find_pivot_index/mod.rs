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
		res = nums[r - 1] + sum_nums(l, r - 1, nums, map);
	}
	map[l][r] = Some(res);
	return res;
}

impl Solution {
	// Time Limit Exceeded
	pub fn pivot_index(nums: Vec<i32>) -> i32 {
		let len = nums.len();
		let mut res = -1;
		let mut sum = 0;
		for num in nums.iter() {
			sum += *num;
		}
		// let mut map: Vec<Vec<Option<i32>>> = vec![vec![None; len]; len]; // 这题有内存限制,	Memory Limit Exceeded
		let mut q = 0;
		for i in 0..len {
			let temp = q;
			if temp == sum - temp - nums[i] {
				res = i as i32;
				break;
			}
			q += nums[i];
		}
		res
	}
}