#![allow(unused)]
use leetcode::base::{num_to_string, string_to_num, Solution};
use leetcode::best_time_to_buy_and_sell_stock_with_cooldown;
use std::mem;

fn main() {
    let mut a = 1;
    let a_ref = &mut a;
    let mut b = 2;
    let b_ref = &mut b;
    // a_ref = b_ref;
    mem::swap(a_ref, b_ref);
    println!("{:?}", a_ref);
    println!("Hello, world!");
}
