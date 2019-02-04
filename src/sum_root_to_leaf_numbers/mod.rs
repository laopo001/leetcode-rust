#![allow(unused)]
use crate::base::{Solution, TreeNode};

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        if let Some(node) = &root {
            res = node.borrow().val;
            Solution::runt(Some(node.clone()), 0, &mut res);
        }
        return res;
    }
    pub fn runt(node: Option<Rc<RefCell<TreeNode>>>, sum: i32, res: &mut i32) {
        if let Some(node) = node {
            Solution::runt(node.borrow().left.clone(), sum + node.borrow().val, res);
            Solution::runt(node.borrow().right.clone(), sum + node.borrow().val, res);
        } else {
            *res += sum;
        }
    }
}
