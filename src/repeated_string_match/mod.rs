#![allow(unused)]
use crate::base::{Solution, TreeNode};

impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let mut s = a.clone();
        let len2 = b.len();
        let mut res = 1;
        while s.len() < len2 || (a.len() != 1 && s.len() <= len2) {
            s = s + a.as_str();
            res = res + 1;
        }
        if s.as_str().contains(b.as_str()) {
            return res;
        }
        return -1;
    }
}