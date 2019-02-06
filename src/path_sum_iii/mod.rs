#![allow(unused)]
use crate::base::{string_to_num, Solution, TreeNode};

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

fn run(root: Option<Rc<RefCell<TreeNode>>>, sum: i32, res: &mut i32, orgin_sum: &i32, level: i32) {
    if root == None {
        return;
    }
    let node = root.unwrap();
    if sum - node.borrow().val == 0 {
        *res += 1;
    }
    run(
        node.borrow().left.clone(),
        sum - node.borrow().val,
        res,
        orgin_sum,
        1,
    );
    run(
        node.borrow().right.clone(),
        sum - node.borrow().val,
        res,
        orgin_sum,
        1,
    );
    if level == 0 {
        *res += Solution::path_sum(node.borrow().left.clone(), *orgin_sum);
        *res += Solution::path_sum(node.borrow().right.clone(), *orgin_sum);
    }
}

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        let mut res = 0;
        run(root, sum, &mut res, &sum, 0);
        return res;
    }
}
