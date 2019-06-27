//use crate::base::Solution;
struct Solution;

impl Solution {
	// 0ms
	pub fn longest_palindromelongest_palindrome(s: String) -> i32 {
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
				res += 1;
			}
		}
		res as i32
	}
}