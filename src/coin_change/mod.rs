use crate::base::Solution;

impl Solution {
	pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
		let len = coins.len();
		let mut dp = vec![vec![std::i32::MAX; amount as usize + 1]; len + 1];
		for i in 1..(len + 1) {
			dp[i][0]=0;
			for j in 1..(amount as usize + 1) {
				if j >= coins[i - 1] as usize {
					// println!(dp[i - 1][j]);
					dp[i][j] = std::cmp::min(dp[i - 1][j] as i64, dp[i][j - coins[i - 1] as usize] as i64 + 1) as i32;
				} else {
					dp[i][j] = dp[i - 1][j];
				}
			}
		}
		// println!("{:?}",dp);
		if std::i32::MAX == dp[len][amount as usize] {
			-1
		} else {
			dp[len][amount as usize]
		}
	}
}

