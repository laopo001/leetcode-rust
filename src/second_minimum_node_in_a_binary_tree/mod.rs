use crate::base::{TreeNode};
use std::rc::Rc;
use std::cell::RefCell;
struct Solution;
fn run(root: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
	//  0 ms, faster than 100.00%
	if let Some(node) = root {
		let val = node.borrow().val;
		if arr.len() == 0 {
			arr.push(val);
		}
		if arr.len() == 1 {
			let last = *arr.last().unwrap();
			if val < last {
				arr.push(val);
			}
			if val > last {
				arr.push(last);
				arr[0] = val;
			}
		}
		if arr.len() == 2 {
			if arr[1] > val {
				arr[0] = arr[1];
				arr[1] = val;
			}
			if arr[0] > val && val != arr[1] {
				arr[0] = val;
			}
		}
		run(node.borrow().left.clone(), arr);
		run(node.borrow().right.clone(), arr);
	}
}

impl Solution {
	pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		let mut arr: Vec<i32> = vec![];
		run(root, &mut arr);

		if arr.len() == 2 {
			return arr[0];
		} else {
			return -1;
		}
	}
}