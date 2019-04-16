use crate::base::{Solution, TreeNode};

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn insert_into_max_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root == None {
            return Some(Rc::new(RefCell::new(TreeNode::new(val))));
        } else {
            if root.clone().unwrap().borrow().val < val {
                let temp = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                temp.clone().unwrap().borrow_mut().left = root;
                return temp;
            }
        }
        root.clone().unwrap().borrow_mut().right =
            Solution::insert_into_max_tree(root.clone().unwrap().borrow().left.clone(), val);
        return root;
    }
}
