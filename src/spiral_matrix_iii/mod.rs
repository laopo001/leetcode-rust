use crate::base::Solution;

impl Solution {
	pub fn spiral_matrix_iii(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
		let matrix = vec![vec![0; c as usize]; r as usize];
		let arr_x: Vec<i32> = vec![1, 0, -1, 0];
		let arr_y: Vec<i32> = vec![0, 1, 0, -1];
		let mut max_x = c0;
		let mut min_x = c0;
		let mut max_y = r0;
		let mut min_y = r0;
		let mut x = c0 + 1;
		let mut y = r0;
		let mut index = 0;
		loop {
			if x >= max_x || x <= min_x || y >= max_y || y <= min_y {

			} else {
				index = (index + 1) % 4;
			}
		}
		vec![vec![0]]
	}
}