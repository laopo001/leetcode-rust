#![allow(unused)]
struct Solution;
use std::cmp::max;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let len = prices.len();
        if len == 0 {
            return 0;
        }
        let mut sell: Vec<i32> = Vec::new(); 
        let mut buy: Vec<i32> = Vec::new(); // 购买成本
        for i in 0..len {
            sell.push(0);
            buy.push(0);
        }
        sell[0] = 0;
        buy[0] = -prices[0];
        for i in 1..len {
            sell[i] = max(sell[i - 1], prices[i] + buy[i - 1]);
            buy[i] = max(buy[i - 1], if i > 1 { sell[i - 2] } else { 0 } - prices[i]);
        }
        // println!("{:?}",sell);
        // println!("{:?}",buy);
        return sell[len - 1];
    }
}
