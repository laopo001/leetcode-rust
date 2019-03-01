use crate::base::Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m == 1 || n == 1 {
            return 1;
        }
        return Solution::unique_paths(m - 1, n) + Solution::unique_paths(m, n - 1);
    }
    pub fn unique_paths2(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp: Vec<Vec<i32>> = vec![];
        for i in 0..n {
            let mut v: Vec<i32> = vec![];
            for j in 0..m {
                if i == 0 || j == 0 {
                    v.push(1);
                } else {
                    v.push(0);
                }
            }
            dp.push(v);
        }
        for i in 1..n {
            for j in 1..m {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }
       return dp[n - 1][m - 1];
    }
}
