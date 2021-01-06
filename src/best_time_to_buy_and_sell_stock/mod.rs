#![allow(unused)]
struct Solution;
use std::cmp::max;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut max_value = std::i32::MAX;
        for item in prices {
            if max_value > item {
                max_value = item;
            } else {
                res = max(res, item - max_value);
            }
        }
        return res;
    }
}
