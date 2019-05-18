use crate::base::Solution;
use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
	pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
		let set: HashSet<i32> = HashSet::from_iter(nums1.into_iter());
		let set2: HashSet<i32> = HashSet::from_iter(nums2.into_iter());
		let mut res: Vec<i32> = vec![];
		for i in set {
			if set2.contains(&i) {
				res.push(i);
			}
		}
		res
	}
}