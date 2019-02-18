#![allow(unused)]
use crate::base::Solution;
use std::cmp::max;

impl Solution {
    pub fn max_profit2(prices: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut sum = 0;
        let mut max_value = std::i32::MAX;
        for i in 0..prices.len() {
            if prices[i] < max_value {
                max_value = prices[i];
            } else {
                res = max(res, prices[i] - max_value);
                if i != prices.len() - 1 && prices[i] > prices[i + 1] {
                    max_value = std::i32::MAX;
                    sum += res;
                    res = 0;
                }
                if i == prices.len() - 1 {
                    sum += res;
                }
            }
        }
        return sum;
    }
}
