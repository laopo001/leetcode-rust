#![allow(non_snake_case)]
struct Solution;
// Runtime: 8 ms, faster than 50.00%
impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        if k == 1 {
            return n;
        }
        let K = k as usize;
        let N = n as usize;
        let mut dp = vec![vec![0; K + 2]; N + 2];
        for m in 1..=N {
            dp[m][0] = 0;
            for k in 1..=K {
                dp[m][k] = dp[m - 1][k] + dp[m - 1][k - 1] + 1;
                if (dp[m][k] >= N) {
                    return m as i32;
                }
            }
        }
        N as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::super_egg_drop(2, 7), 4);
}
