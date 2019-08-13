use crate::base::{Solution, TreeNode};

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
	pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		let mut res = std::i32::MAX;
		Solution::run2(root, &mut res);
		res
	}
	fn run2(node: Option<Rc<RefCell<TreeNode>>>, res: &mut i32) {
		if let Some(n) = node {
			let val = n.borrow().val;
			if let Some(l) = n.borrow().left.clone() {
				let min = Solution::get_right(l.clone());
				*res = std::cmp::min(*res, val - min.borrow().val);
				Solution::run2(Some(l), res);
			}
			if let Some(r) = n.borrow().right.clone() {
				let min = Solution::get_left(r.clone());

				*res = std::cmp::min(*res, min.borrow().val - val);
				Solution::run2(Some(r), res);
			}
		}
	}
	fn get_left(node: Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
		if let Some(t) = node.borrow().left.clone() {
			Solution::get_left(t)
		} else {
			node.clone()
		}
	}
	fn get_right(node: Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
		if let Some(t) = node.borrow().right.clone() {
			Solution::get_right(t)
		} else {
			node.clone()
		}
	}
}

#[test]
fn test() {
	use std::rc::Rc;
	let x = Rc::new(10);
	let x_ptr = Rc::into_raw(x.clone());
	let x_ptr2 = Rc::into_raw(x.clone());
	unsafe {
		let u = std::mem::transmute::<*const i32, usize>(x_ptr);
		println!("{}", u);
		let u2 = std::mem::transmute::<*const i32, usize>(x_ptr2);
		println!("{}", u2);
		assert_eq!(u, u2);
	}
	assert_eq!(unsafe { *x_ptr }, 10);
}