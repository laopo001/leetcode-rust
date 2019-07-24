use crate::base::Solution;

impl Solution {
	pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
		let arr_x: Vec<i32> = vec![1, 0, -1, 0];
		let arr_y: Vec<i32> = vec![0, 1, 0, -1];
		let mut len_y = matrix.len();
		if len_y == 0 {
			return vec![];
		}
		if matrix[0].len() == 0 {
			return vec![];
		}
		let mut max_x: usize = matrix[0].len() - 1;
		let mut max_y: usize = len_y - 1;
		let mut min_x: usize = 0;
		let mut min_y: usize = 0;
		let mut i: usize = 0;
		let mut x: usize = 0;
		let mut y: usize = 0;
		let mut res: Vec<i32> = vec![];
		loop {
			if y >= min_y && y <= max_y && x > min_x && x < max_y {
				x += arr_x[i] as usize;
				y += arr_y[i] as usize;
				res.push(matrix[y][x]);
			} else {
				if i == 0 {
					min_y += 1;
				}
				if i == 1 {
					max_x -= 1;
				}
				if i == 2 {
					max_y -= 1;
				}
				if i == 3 {
					min_x += 1;
				}
				if max_x == min_x || max_y == min_y {
					break;
				}
				i = (i + 1) % 4;
			}
		}
		res
	}
}