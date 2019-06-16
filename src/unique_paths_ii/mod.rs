use crate::base::Solution;

impl Solution {
	pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
		let m = obstacle_grid[0].len();
		let n = obstacle_grid.len();
		let mut dp: Vec<Vec<i32>> = vec![vec![0; m]; n];
		if obstacle_grid[0][0] == 1 {
			return 0;
		}
		dp[0][0] = 1;
		for i in 1..n {
			if obstacle_grid[i][0] == 1 {
				continue;
			}
			dp[i][0] = dp[i - 1][0];
		}
		for i in 1..m {
			if obstacle_grid[0][i] == 1 {
				continue;
			}
			dp[0][i] = dp[0][i - 1];
		}
		for i in 1..n {
			for j in 1..m {
				if obstacle_grid[i][j] == 1 {
					continue;
				}
				dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
			}
		}
		// println!("{:?}", dp);
		return dp[n - 1][m - 1];
	}
}