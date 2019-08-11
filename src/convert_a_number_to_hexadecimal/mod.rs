use crate::base::Solution;
use std::collections::HashMap;

fn num_to_vec(n: i64, d: i64) -> Vec<i64> {
	let mut res: Vec<i64> = vec![];
	let mut n = n;
	while n != 0 {
		let t = n % d;
		res.push(t);
		n = (n - t) / d;
	}
//	res.sort_by(|a, b| { b.cmp(a) });
	res.reverse();
	return res;
}

impl Solution {
	// 0 ms, faster than 100.00%
	pub fn to_hex(mut num: i32) -> String {
		let mut num: i64 = num as i64;
		if num == 0 {
			return "0".to_string();
		}
		if num < 0 {
			num = -(2*std::i32::MIN as i64 - num);
		}
		let mut map: HashMap<i64, &str> = HashMap::new();
		map.insert(0, "0");
		map.insert(1, "1");
		map.insert(2, "2");
		map.insert(3, "3");
		map.insert(4, "4");
		map.insert(5, "5");
		map.insert(6, "6");
		map.insert(7, "7");
		map.insert(8, "8");
		map.insert(9, "9");
		map.insert(10, "a");
		map.insert(11, "b");
		map.insert(12, "c");
		map.insert(13, "d");
		map.insert(14, "e");
		map.insert(15, "f");
		let arr = num_to_vec(num, 16);
//		println!("{:?}", arr);
		let mut res = "".to_string();
		for i in arr {
			res += map.get(&i).unwrap();
		}
		return res;
	}
}