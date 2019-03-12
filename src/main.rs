#![allow(unused)]
use leetcode::base::{num_to_string, string_to_num, Solution};
use leetcode::longest_palindromic_substring;
use std::mem;

fn main() {
    let s = Solution::longest_palindrome2("aabaaa".to_string());
    println!("{:?}", s);
    println!("Hello, world!");
}
