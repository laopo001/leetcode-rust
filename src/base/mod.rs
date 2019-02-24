#![allow(unused)]

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;

pub fn index_of_vec<T: PartialEq>(v: &Vec<T>, i: T) -> Option<usize> {
    for item in 0..v.len() {
        if i == v[item] {
            return Some(item);
        }
    }
    return None;
}

pub fn num_to_string(n: i32, d: i32) -> String {
    let mut res = "".to_string();
    let mut n = n;
    while n != 0 {
        let t = n % d;
        res = t.to_string() + res.as_str();
        n = (n - t) / d;
    }
    return res.to_string();
}

pub fn string_to_num(s: String, d: i32) -> i32 {
    let mut res = 0;
    let bytes = s.as_bytes();
    for i in 0..bytes.len() {
        let index = bytes.len() - 1 - i;
        res += (bytes[i] as i32 - 48) * d.pow(index as u32);
    }
    return res;
}

#[test]
fn test_string_to_num() {
    assert_eq!(string_to_num("12".to_string(), 10), 12);
}
