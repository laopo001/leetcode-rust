struct Solution;

impl Solution {
	pub fn height_checker(heights: Vec<i32>) -> i32 {
		let mut res = 0;
		let mut arr = heights.clone();
		arr.sort();
		for i in 0..heights.len() {
			if heights[i] != arr[i] {
				res += 1;
			}
		}
		res
	}
}