#![allow(unused)]
use crate::base::{string_to_num, Solution, TreeNode};

use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

fn run(root: Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
    if root == None {
        return 0;
    }
    let node = root.unwrap();

    let mut l = run(node.borrow().left.clone(), res);
    let mut r = run(node.borrow().right.clone(), res);
    if node.borrow().left.clone() != None
        && node.borrow().val == node.borrow().left.clone().unwrap().borrow().val
    {
        l += 1;
    } else {
        l = 0;
    }
    if node.borrow().right.clone() != None
        && node.borrow().val == node.borrow().right.clone().unwrap().borrow().val
    {
        r += 1;
    } else {
        r = 0;
    }
    *res = max(*res, l + r);
    return max(l, r);
}

impl Solution {
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        run(root, &mut res);
        return res;
    }
}
