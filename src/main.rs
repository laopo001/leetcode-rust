#![allow(unused)]
use leetcode::base::Solution;
use leetcode::split_array_largest_sum;

fn main() {
    Solution::split_array(vec![1, 2147483646], 2);
    let mut a: i32 = 0;
    let mut b = &mut a;
    *b = 2;
    println!("{}", a);
    println!("Hello, world!");
}
