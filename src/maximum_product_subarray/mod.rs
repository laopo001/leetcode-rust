struct Solution;

impl Solution {
	pub fn max_product(nums: Vec<i32>) -> i32 {
		use std::cmp::{min, max};
		let len = nums.len();
		let mut max_values = vec![0; len];
		let mut min_values = vec![0; len];
		let mut res = std::i32::MIN;
		max_values[0] = nums[0];
		min_values[0] = nums[0];
		for i in 1..len {
			max_values[i] = max(nums[i] * max_values[i - 1], max(nums[i], nums[i] * min_values[i - 1]));
			min_values[i] = min(nums[i] * max_values[i - 1], min(nums[i], nums[i] * min_values[i - 1]));
			res = max(max_values[i], res);
		}
		res
	}
}