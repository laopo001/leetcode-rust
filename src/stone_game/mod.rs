use crate::base::Solution;
use std::cmp::max;
fn run(piles: &Vec<i32>, l: usize, r: usize) -> i32 {
    if l == r {
        return piles[l];
    }
    return max(
        piles[l] - run(&piles, l + 1, r),
        piles[r] - run(&piles, l, r - 1),
    );
}

impl Solution {
    pub fn stone_game1(piles: Vec<i32>) -> bool {
        return run(&piles, 0, piles.len() - 1) > 0;
    }
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let len = piles.len();
        let mut dp: Vec<Vec<i32>> = vec![];
        for i in 0..len {
            let mut v:Vec<i32> = vec![];
            for _ in 0..len {
                v.push(0);
            }
            dp.push(v);
            dp[i][i] = piles[i];
        }
        for dis in 1..len {
            for i in 0..(len - dis) {
                dp[i][i + dis] = max(
                    piles[i] - dp[i + 1][i + dis],
                    piles[i + dis] - dp[i][i + dis - 1],
                );
            }
        }
        return dp[0][len-1] > 0;
    }
}
