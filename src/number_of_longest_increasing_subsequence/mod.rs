struct Solution;

impl Solution {
	pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
		use std::cmp::max;
		let len = nums.len();
		if len == 0 {
			return 0;
		}
		let mut dp = vec![1; len];
		let mut cnt = vec![1; len];
		let mut res = 0;
		let mut sum = 1;
		for i in 0..len {
			let mut first = false;
			for j in 0..i {
				if nums[j] < nums[i] {
					if dp[i] == dp[j] + 1 {
						cnt[i] += cnt[j];
					}
					if dp[i] < dp[j] + 1 {
						dp[i] = dp[j] + 1;
						cnt[i] = cnt[j];
					}
//					dp[i] = max(dp[j] + 1, dp[i]);
				}
			}
//			res = max(dp[i], res);
			if res < dp[i] {
				res = dp[i];
				sum = cnt[i];
			} else if res == dp[i] {
				sum += cnt[i];
			}
		}
		sum
	}
}