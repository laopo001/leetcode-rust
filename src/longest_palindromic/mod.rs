use crate::base::Solution;

impl Solution {
	// 0ms
	pub fn longest_palindrome3(s: String) -> i32 {
		let mut map: Vec<usize> = vec![0; 256];
		for i in s.as_bytes() {
			let item = &mut map[(*i as usize)];
			*item += 1;
		}
		let mut res = 0;
		let mut b = true;
		for x in map.iter() {
			let temp = *x % 2;
			res += x - temp;
			if b && temp != 0 {
				b = false;
				res+=1;
			}
		}
		res as i32
	}
}