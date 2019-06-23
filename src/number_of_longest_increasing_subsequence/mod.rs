use crate::base::Solution;

impl Solution {
	pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
		use std::cmp::max;
		let len = nums.len();
		if len == 0 {
			return 0;
		}
		let mut dp = vec![1; len];
		let mut res = 0;
		let mut sum = 1;
		for i in 0..len {
			for j in 0..i {
				if nums[j] < nums[i] {
					dp[i] = max(dp[j] + 1, dp[i]);
				}
			}
			if dp[i] > res {
				res = dp[i];
			} else if dp[i] == res {
				sum += 1;
			}
		}
		println!("{:?}", dp);
		sum
	}
}