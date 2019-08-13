use crate::base::Solution;

impl Solution {
	pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
		let mut left: Vec<i32> = vec![];
		let mut right: Vec<i32> = vec![];
		for item in a.iter() {
			if *item % 2 == 0 {
				left.push(*item);
			} else {
				right.push(*item);
			}
		}
		left.extend(right);
		left
	}
}