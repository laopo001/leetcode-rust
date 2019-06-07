use crate::base::{Solution, TreeNode};

use std::rc::Rc;
use std::cell::RefCell;

fn run(root: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
	if let Some(node) = root {
		if node.borrow().left.is_none() && node.borrow().right.is_none() {
			arr.push(node.borrow().val)
		} else {
			if node.borrow().left.is_some() {
				run(node.borrow().left.clone(), arr);
			}
			if node.borrow().right.is_some() {
				run(node.borrow().right.clone(), arr);
			}
		}
	}
}

impl Solution {
	// 0ms
	pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
		let mut res: Vec<i32> = vec![];
		run(root1, &mut res);
		let mut res2: Vec<i32> = vec![];
		run(root2, &mut res2);
		if res.len() != res2.len() {
			return false;
		}
		for i in 0..res.len() {
			if res[i] != res2[i] {
				return false;
			}
		}
		true
	}
}