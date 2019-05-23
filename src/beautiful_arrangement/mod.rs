use crate::base::Solution;

impl Solution {
	pub fn count_arrangement(n: i32) -> i32 {
		let mut res = 0;
		let mut arr: Vec<i32> = vec![0, n + 1];
		for i in 1..(n + 1) {
			arr[i] = i;
		}
		res
	}
}