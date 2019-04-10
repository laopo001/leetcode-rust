use crate::base::{Solution, TreeNode};

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root == None {
            let n = TreeNode::new(val);
            return Some(Rc::new(RefCell::new(n)));
        } else {
            let root_w = root.clone().unwrap();
            if root_w.borrow().val > val {
                let left = root_w.borrow().left.clone();

                root_w.borrow_mut().left = Solution::insert_into_bst(left, val);
            }
            if root_w.borrow().val < val {
                let right = root_w.borrow().right.clone();

                root_w.borrow_mut().right = Solution::insert_into_bst(right, val);
            }
        }
        return root;
    }
}
