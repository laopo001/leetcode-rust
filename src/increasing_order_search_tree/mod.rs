use crate::base::{Solution, TreeNode};

use std::cell::RefCell;
use std::rc::Rc;
// use std::borrow::Borrow;

fn bst_to_arr(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    if root.is_some() {
        let t = root.unwrap();
        res.push(t.borrow().val);
        let mut l = bst_to_arr(t.borrow().left.clone());
        let r = bst_to_arr(t.borrow().right.clone());
        l.extend(res);
        l.extend(r);
        res = l;
    }
    res
}

impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut v = bst_to_arr(root);
        let mut root: Option<Rc<RefCell<TreeNode>>> = None;
        if v.len() == 0 {
            return None;
        } else {
            let item = v.pop().unwrap();
            v.reverse();
            let mut node_o = Some(Rc::new(RefCell::new(TreeNode::new(item))));
            for item in v {
                let mut node = TreeNode::new(item);
                node.right = node_o.clone();
                node_o = Some(Rc::new(RefCell::new(node)));
            }
            root = node_o;
        }
        root
    }
}
