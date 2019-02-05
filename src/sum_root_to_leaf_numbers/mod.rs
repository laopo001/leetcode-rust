#![allow(unused)]
use crate::base::{string_to_num, Solution, TreeNode};

use std::cell::RefCell;
use std::rc::Rc;


impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        if let Some(node) = &root {
            let sum = "".to_string();
            Solution::runt(Some(node.clone()), sum, &mut res);
        }
        return res;
    }
    pub fn runt(node: Option<Rc<RefCell<TreeNode>>>, sum: String, res: &mut i32) {
        let node = node.unwrap();
        if node.borrow().left != None {
            Solution::runt(
                node.borrow().left.clone(),
                sum.to_string() + node.borrow().val.to_string().as_str(),
                res,
            );
        }
        if node.borrow().right != None {
            Solution::runt(
                node.borrow().right.clone(),
                sum.to_string() + node.borrow().val.to_string().as_str(),
                res,
            );
        }
        if node.borrow().left == None && node.borrow().right == None {
            *res += string_to_num(sum + node.borrow().val.to_string().as_str(), 10);
        }
    }
}
