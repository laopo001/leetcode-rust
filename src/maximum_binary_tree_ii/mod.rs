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
        }
        let root = root.unwrap();
        if root.borrow().val < val {
            let temp = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            temp.clone().unwrap().borrow_mut().left = Some(root);
            return temp;
        }
        let mut root_mut = root.borrow_mut();
        root_mut.right = Solution::insert_into_max_tree(root_mut.right.clone(), val);
        drop(root_mut);
        return Some(root);
    }
}
