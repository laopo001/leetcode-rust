#![allow(unused)]
use leetcode::base::{num_to_string, string_to_num, Solution};
use leetcode::best_time_to_buy_and_sell_stock_with_cooldown;

fn main() {
    Solution::max_profit3(vec![2, 6, 5, 0, 3]);
    let mut s = num_to_string(5, 2);

    let mut temp = String::new();

    for c in s.bytes() {
        if c == b'0' {
            temp.push('1');
        } else {
            temp.push('0');
        }
    }

    let mut res = string_to_num(temp, 2);
    println!("{:?}", res);
    println!("Hello, world!");
}
