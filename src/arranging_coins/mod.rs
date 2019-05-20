use crate::base::Solution;

impl Solution {
	pub fn arrange_coins(n: i32) -> i32 {
		if n == 1 { return 1; }
		let mut t = (((n as f64) * 2.0).sqrt() - 1.0) as i64;
		while (n as i64) - t * (t + 1) / 2 > t {
			t += 1;
		}
		return t as i32;
	}
}