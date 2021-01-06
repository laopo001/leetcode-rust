struct Solution;

impl Solution {
	// 24 ms, faster than 100.00%
	pub fn find_lhs(nums: Vec<i32>) -> i32 {
		use std::collections::HashMap;
		let mut map: HashMap<i32, i32> = HashMap::new();
		for num in nums {
			if let Some(value) = map.get_mut(&num) {
				*value += 1;
			} else {
				map.insert(num, 1);
			}
		}
		#[derive(Debug)]
		struct Item {
			k: i32,
			v: i32,
		}
		let mut arr: Vec<Item> = vec![];
		for (k, v) in map {
			arr.push(Item {
				k,
				v,
			});
		}
		arr.sort_by(|a, b| { return a.k.cmp(&b.k); });
		// println!("{:?}", arr);
		let mut max_res = 0;
		for i in 0..arr.len() {
			if i + 1 >= arr.len() {
				break;
			} else {
				if arr[i].k + 1 == arr[i+1].k {
					max_res = std::cmp::max(max_res, arr[i].v + arr[i+1].v);
				}
			}
		}
		return max_res;
	}
}