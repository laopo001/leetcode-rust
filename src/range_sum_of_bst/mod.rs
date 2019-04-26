use crate::base::{Solution, TreeNode};


use std::cell::RefCell;
use std::rc::Rc;
fn run(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32, res: &mut i32) {
    if root.is_none() {
        return;
    }
    let root_value = root.unwrap();
    if root_value.borrow().val > r {
        run(root_value.borrow().left.clone(), l, r, res);
    }
    if root_value.borrow().val < l {
        run(root_value.borrow().right.clone(), l, r, res);
    }
    if root_value.borrow().val <= r && root_value.borrow().val >= l {
        *res += root_value.borrow().val;
        run(root_value.borrow().left.clone(), l, r, res);
        run(root_value.borrow().right.clone(), l, r, res);
    }
}

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
        let mut res = 0;
        run(root, l, r, &mut res);
        res
    }
}