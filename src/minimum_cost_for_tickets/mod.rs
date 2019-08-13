use crate::base::Solution;

impl Solution {
	pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
		use std::cmp::{min,max};
		let mut dp: Vec<i32> = vec![std::i32::MAX; 366];
		for day in days.iter() {
			dp[*day as usize] = 0;
		}
		dp[0] = 0;
		for i in 1..366 as usize {
			if dp[i] == std::i32::MAX {
				dp[i] = dp[i - 1];
			} else {
				let mut t = dp[i - 1] + costs[0];
				// println!("{}", i as i32 - 7);
				t = min(t, costs[1] + dp[max(0, i  as i32  - 7) as usize]);
				t = min(t, costs[2] + dp[max(0, i  as i32 - 30) as usize]);
				dp[i] = t;
			}
		}
		return dp[days[days.len() - 1] as usize];
	}
}