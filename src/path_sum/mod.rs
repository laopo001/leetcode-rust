#![allow(unused)]
use crate::base::{string_to_num, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
       //   println!("{:?}", sum);
        if root == None {
            return false;
        }
        let node = root.unwrap();

        if node.borrow().right == None && node.borrow().left == None && sum - node.borrow().val == 0 {
            return true;
        }
        return Solution::has_path_sum(node.borrow().left.clone(), sum - node.borrow().val)
            || Solution::has_path_sum(node.borrow().right.clone(), sum - node.borrow().val);
    }
}
