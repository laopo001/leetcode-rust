use crate::base::Solution;

impl Solution {
	// 8 ms, faster than 100.00%
	pub fn spiral_matrix_iii(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
		let matrix = vec![vec![0; c as usize]; r as usize];
		let arr_x: Vec<i32> = vec![1, 0, -1, 0];
		let arr_y: Vec<i32> = vec![0, 1, 0, -1];
		let len_y = std::cmp::max(r - r0, r0);
		let len_x = std::cmp::max(c - c0, c0);
		let mut max_x = c0;
		let mut min_x = c0;
		let mut max_y = r0;
		let mut min_y = r0;
		let mut x = c0;
		let mut y = r0;
		let mut index = 0;
		let mut res = vec![vec![r0, c0]];
		loop {
			x = x + arr_x[index];
			y = y + arr_y[index];
			if x >= 0 && x < c && y >= 0 && y < r {
				res.push(vec![y, x]);
			}
//			res.push(vec![y, x]);
			if x > max_x || x < min_x || y > max_y || y < min_y {
				index = (index + 1) % 4;
				if x > max_x {
					max_x += 1;
				}
				if x < min_x {
					min_x -= 1;
				}
				if y > max_y {
					max_y += 1;
				}
				if y < min_y {
					min_y -= 1;
				}
			}
			if max_x - c0 >= len_x + 1 && max_y - r0 >= len_y + 1 {
				break;
			}
		}
		return res;
	}
}