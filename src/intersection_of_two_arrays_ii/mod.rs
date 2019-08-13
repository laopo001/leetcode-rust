use crate::base::Solution;
use std::collections::HashMap;

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
	pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
		let map1 = vec_to_map(&nums1);
		let map2 = vec_to_map(&nums2);
		let mut res: Vec<i32> = vec![];
		for (key, value) in map1 {
			if map2.contains_key(&key) {
				let v = map2.get(&key).unwrap();
				let temp = std::cmp::min(*v, value);
				for _ in 0..temp {
					res.push(key);
				}
			}
		}
		res
	}
}