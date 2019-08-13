use crate::base::Solution;

impl Solution {
	pub fn count_substrings(s: String) -> i32 {
		let len = s.len();
		let arr = s.as_bytes();
		let mut dp = vec![vec![0; len]; len];
		let mut res = 0;
		for i in 0..len {
			dp[i][i] = 1;
			res += 1;
		}
		for i in 1..len {
			if arr[i] == arr[i - 1] {
				dp[i - 1][i] = 1;
				res += 1;
			}
		}
		// println!("{:?}", dp);
		for j in 2..len {
			for i in 0..=(j - 2) {
				// println!("{},{},{}", i, j, dp[i + 1][j - 1]);
				if arr[i] == arr[j] && dp[i + 1][j - 1] == 1 {
					dp[i][j] = 1;
					res += 1;
				}
			}
		}

		res
	}
}