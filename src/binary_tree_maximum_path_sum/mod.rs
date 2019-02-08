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
    if node.borrow().left.clone() == None && node.borrow().right.clone() == None {
        *res = max(*res, node.borrow().val);
        return node.borrow().val;
    }
    let l = run(node.borrow().left.clone(), res);
    let r = run(node.borrow().right.clone(), res);
    let maxValue = max(
        l + r + node.borrow().val,
        max(
            node.borrow().val,
            max(l + node.borrow().val, r + node.borrow().val),
        ),
    );
    *res = max(*res, maxValue);
    return max(
            node.borrow().val,
            max(l + node.borrow().val, r + node.borrow().val),
        );
}

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = std::i32::MIN;
        run(root, &mut res);
        return res;
    }
}
