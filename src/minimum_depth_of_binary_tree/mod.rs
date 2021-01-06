#![allow(unused)]
use crate::base::{TreeNode};
use std::cell::RefCell;
use std::i32::MAX;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = &root {
            let mut max = MAX;
            Solution::run(Some(node.clone()), 1, &mut max);
            return max;
        }
        return 0;
    }
    fn run(root: Option<Rc<RefCell<TreeNode>>>, depth: i32, max: &mut i32) {
        if let Some(node) = &root {
            if depth < *max && node.borrow().right == None && node.borrow().left == None {
                *max = depth;
            }
            Solution::run(node.borrow().left.clone(), depth + 1, max);
            Solution::run(node.borrow().right.clone(), depth + 1, max);
        }
    }
}

#[test]
fn test() {
    let root = Rc::new(RefCell::new(TreeNode::new(0)));

    assert_eq!(Solution::min_depth(Some(root)), 1);
}
