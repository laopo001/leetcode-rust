use crate::base::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn merge_trees(
        t1: Option<Rc<RefCell<TreeNode>>>,
        t2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if t1.is_none() && t2.is_none() {
            return None;
        } else if t1.is_some() && t2.is_none() {
            let t_clone = t1.clone().unwrap();
            let t = t_clone.borrow().val;
            let mut node = TreeNode::new(t);
            node.left = Solution::merge_trees(t_clone.borrow().left.clone(), None);
            node.right = Solution::merge_trees(t_clone.borrow().right.clone(), None);
            return Some(Rc::new(RefCell::new(node)));
        } else if t1.is_none() && t2.is_some() {
            let t2_clone = t2.clone().unwrap();
            let t = t2_clone.borrow().val;
            let mut node = TreeNode::new(t);
            node.left = Solution::merge_trees(None, t2_clone.borrow().left.clone());
            node.right = Solution::merge_trees(None, t2_clone.borrow().right.clone());
            return Some(Rc::new(RefCell::new(node)));
        } else {
            let t_clone = t1.clone().unwrap();
            let t2_clone = t2.clone().unwrap();
            let t = t_clone.borrow().val + t2_clone.borrow().val;
            let mut node = TreeNode::new(t);
            node.left = Solution::merge_trees(
                t_clone.borrow().left.clone(),
                t2_clone.borrow().left.clone(),
            );
            node.right = Solution::merge_trees(
                t_clone.borrow().right.clone(),
                t2_clone.borrow().right.clone(),
            );
            return Some(Rc::new(RefCell::new(node)));
        }
    }
}
