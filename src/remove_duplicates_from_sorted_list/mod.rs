use crate::base::{Solution, ListNode};

impl Solution {
	// 0 ms, faster than 100.00%
	pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let mut arr: Vec<i32> = vec![];
		let mut head = head;
		while let Some(node) = head {
			arr.push(node.val);
			head = node.next;
		}
		let mut res = None;
		let mut p: Option<i32> = None;
		while arr.len() != 0 {
			let mut q = arr.pop().unwrap();
			if p.is_none() {
				p = Some(q);
			} else {
				unsafe {
					if p.unwrap() == q {
						continue;
					} else {
						p = Some(q);
					}
				}
			}
			let mut node = ListNode::new(q);
			node.next = res;
			res = Some(Box::new(node));
		}
		res
	}
	pub fn delete_duplicates2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let mut root: *mut Box<ListNode> = std::ptr::null_mut();
		let mut node = head.clone();
		loop {
			match node.as_mut() {
				Some(v) => {
					if root.is_null() {
						root = v;
					}

					let mut temp = v.next.clone();

					loop {
						match temp.as_mut() {
							Some(v2) => {
								if v.val == v2.val {
									temp = v2.next.clone();
									// println!("{:?}", temp);
									// v.next = temp.clone();
								} else {
									break;
								}
							}
							None => {
								break;
							}
						}
					}
					// println!("{:?}", temp);
					v.next = temp;
					node = v.next.clone();
				}
				None => {
					break;
				}
			}
		}
		unsafe {
			if root.is_null() {
				return None;
			} else {
				let t = (*root).clone();
				return Some(t);
			}
		}
	}
}