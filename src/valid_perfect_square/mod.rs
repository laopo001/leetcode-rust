struct Solution;

impl Solution {
	// 0 ms, faster than 100.00%
	pub fn is_perfect_square(num: i32) -> bool {
		let mut i: i64 = 0;
		let mut res: i64 = i * i;
		if 0 == num {
			return false;
		}
		while res < num as i64 {
			i += 1;
			res = i * i;
			if res == num as i64 {
				return true;
			}
		}
		return false;
	}
}