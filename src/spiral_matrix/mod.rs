struct Solution;

impl Solution {
	// 0 ms, faster than 100.00%
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
		// println!("{},{},{}", max_x,max_y,1);
		let mut min_x: usize = 0;
		let mut min_y: usize = 0;
		let mut i: usize = 0;
		let mut x: usize = 0;
		let mut y: usize = 0;
		let mut res: Vec<i32> = vec![matrix[y][x]];
		loop {
			// println!("{},{},{}", x, y, i);
			let t_x = x + arr_x[i] as usize;
			let t_y = y + arr_y[i] as usize;
			if t_y >= min_y && t_y <= max_y && t_x >= min_x && t_x <= max_x {
				x = t_x;
				y = t_y;
				res.push(matrix[y][x]);
			} else {
				if i == 0 {
					min_y += 1;
				}
				if i == 1 {
					if max_x as i32 - 1 < 0 {
						break;
					}
					max_x -= 1;
				}
				if i == 2 {
					if max_y as i32 - 1 < 0 {
						break;
					}
					max_y -= 1;
				}
				if i == 3 {
					min_x += 1;
				}
				if max_x < min_x || max_y < min_y {
					break;
				}
				i = (i + 1) % 4;
			}
		}
		res
	}
}