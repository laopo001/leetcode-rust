use crate::base::Solution;

impl Solution {
	// dp
	pub fn length_of_lis(nums: Vec<i32>) -> i32 {
		use std::cmp::max;
		let len = nums.len();
		let mut dp = vec![1; len];
		let mut res = 0;
		for i in 0..len {
			for j in 0..i {
				if nums[j] < nums[i] {
					dp[i] = max(dp[j] + 1, dp[i]);
				}
			}
			res = max(dp[i], res);
		}
		//		println!("{:?}", dp);
		res
	}
}
