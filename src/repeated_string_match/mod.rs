#![allow(unused)]
use crate::base::{Solution, TreeNode};

impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let mut s = a.clone();
        let mut res = 1;
        while s.len() < b.len() {
            s = s + a.as_str();
            res = res + 1;
        }
        if s.as_str().contains(b.as_str()) {
            return res;
        } else {
            s = s + a.as_str();
            res = res + 1;
        }
        if s.as_str().contains(b.as_str()) {
            return res;
        }
        return -1;
    }
}
