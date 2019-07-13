use crate::base::{Solution, ListNode};

impl Solution {
	pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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