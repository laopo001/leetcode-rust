// use crate::base::Solution;
struct Solution;

impl Solution {
	pub fn count_arrangement(n: i32) -> i32 {
		let mut res = 0;
		let mut arr: Vec<i32> = vec![0; n as usize + 1];
		// println!("{:?}",arr);
		Solution::helper(n as usize, &mut arr, 1, &mut res);
		res
	}
	fn helper(n: usize, arr: &mut Vec<i32>, pos: usize, res: &mut i32) {
		if pos > n {
			*res += 1;
			return;
		}
		for i in 1..(n + 1) {
			if arr[i] == 0 && (i % pos == 0 || pos % i == 0) {
				arr[i] = 1;
				Solution::helper(n, arr, pos + 1, res);
				arr[i] = 0;
			}
		}
	}
}