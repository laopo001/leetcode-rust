#![allow(unused)]
use std::cell::RefCell;
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::i32;
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

#[derive(Debug)]
struct Solution;

impl Solution {
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = &root {
            return Solution::is_unival_tree2(Some(node.clone()), node.borrow().val);
        }
        return true;
    }
    pub fn is_unival_tree2(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> bool {
        if let Some(node) = &root {
            // let res = node.borrow().val != val;
            if node.borrow().val != val {
                return false;
            }
            let left_res = Solution::is_unival_tree2(node.borrow().left.clone(), node.borrow().val);
            let right_res = Solution::is_unival_tree2(node.borrow().right.clone(), node.borrow().val);
            return left_res && right_res;
        }
        return true;
    }
}