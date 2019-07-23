use crate::base::Solution;

impl Solution {
	pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
		let arr_x = [1, 0, -1, 0];
		let arr_y = [0, 1, 0, -1];
		let len_y = matrix.len();
		if len_y == 0{
			return vec![];
		}
		let len_x= matrix[0].len();

		vec![0]
	}
}