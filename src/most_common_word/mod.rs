use crate::base::Solution;
use std::collections::HashMap;

impl Solution {
	// 0 ms, faster than 100.00%
	pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
		let mut temp = "".to_string();
		for item in paragraph.as_bytes() {
			if *item == b','
				|| *item == b'.'
				|| *item == b'!'
				|| *item == b'?'
				|| *item == b';'
				|| *item == b'\''{
				temp += " ";
				continue;
			}
			temp += &String::from_utf8(vec![*item]).unwrap();
		}
		let paragraph = temp;
		let mut arr: Vec<&str> = paragraph.split(" ").collect();
		let mut map: HashMap<String, i32> = HashMap::new();
		for i in 0..arr.len() {
			if arr[i] == "" {
				continue;
			}
			let s = arr[i].to_lowercase();
			if map.contains_key(&s) {
				*map.get_mut(&s).unwrap() += 1;
			} else {
				map.insert(s, 1);
			}
		}
		// println!("{:?}", map);
		let mut max = std::i32::MIN;
		let mut res = "".to_string();
		for (s, size) in map {
			if max < size && !banned.contains(&s.to_string()) {
				res = s;
				max = size;
			}
		}
		res
	}
}