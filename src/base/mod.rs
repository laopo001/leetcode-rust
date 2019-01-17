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
