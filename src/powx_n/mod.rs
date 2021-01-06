struct Solution;
use std::collections::HashMap;


impl Solution {
	// 0 ms, faster than 100.00%
	pub fn my_pow(x: f64, n: i32) -> f64 {
		if n == -1 {
			return 1.0 / x;
		}
		if n == 0 {
			return 1.0;
		}
		if n == 1 {
			return x;
		}
		let q = n / 2;
		let p = n % 2;
		let z = Solution::my_pow(x, q);
		return z * z * Solution::my_pow(x, p);
	}
}