use crate::base::Solution;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;

fn vec_to_map(arr: &Vec<i32>) -> HashMap<i32, usize> {
	let mut map1: HashMap<i32, usize> = HashMap::new();
	arr.iter().for_each(|x| {
		if map1.contains_key(x) {
			map1.insert(*x, *map1.get(x).unwrap() + 1);
		} else {
			map1.insert(*x, 1);
		};
	});
	map1
}


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
	pub fn intersection2(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
		let map1 = vec_to_map(&nums1);
		let map2 = vec_to_map(&nums2);
		let mut res: Vec<i32> = vec![];
		for (key, _value) in map1 {
			if map2.contains_key(&key) {
				res.push(key);
			}
		}
		res
	}
}