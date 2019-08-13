use crate::base::Solution;
use std::cmp::max;
impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n as usize + 1]; m as usize + 1];
        for s in strs {
            let mut one = 0;
            let mut zero = 0;
            for c in s.chars() {
                if c == '0' {
                    zero += 1;
                } else {
                    one += 1;
                }
            }

            for i in (zero..(m + 1)).rev() {
                for j in (one..(n + 1)).rev() {
                    dp[i as usize][j as usize] = max(
                        dp[i as usize][j as usize],
                        dp[(i - zero) as usize][(j - one) as usize] + 1,
                    );
                }
            }
        }
        return dp[m as usize][n as usize];
    }
}
