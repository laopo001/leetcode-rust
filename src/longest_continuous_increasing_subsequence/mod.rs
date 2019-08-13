use crate::base::Solution;

impl Solution {
	pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
		use std::cmp::max;
		let len = nums.len();
		if len == 0 {
			return 0;
		}
		let mut dp = vec![1; len];
		let mut res = 1;
		for i in 1..len {
			if nums[i] > nums[i - 1] {
				dp[i] = dp[i - 1] + 1;
				res = max(dp[i], res);
			}
		}
		res
	}
}