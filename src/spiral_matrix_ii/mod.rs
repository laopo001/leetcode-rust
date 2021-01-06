struct Solution;

impl Solution {
	// 0 ms, faster than 100.00%
	pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
		let arr_x: Vec<i32> = vec![1, 0, -1, 0];
		let arr_y: Vec<i32> = vec![0, 1, 0, -1];
		let mut matrix = vec![vec![1; n as usize]; n as usize];
		if n == 0 {
			return vec![];
		}
		let mut max_x: usize = n as usize - 1;
		let mut max_y: usize = n as usize - 1;
		let mut min_x: usize = 0;
		let mut min_y: usize = 0;
		let mut i: usize = 0;
		let mut x: usize = 0;
		let mut y: usize = 0;
		let mut c = 1;
		loop {
			let t_x = x + arr_x[i] as usize;
			let t_y = y + arr_y[i] as usize;
			if t_y >= min_y && t_y <= max_y && t_x >= min_x && t_x <= max_x {
				c += 1;
				x = t_x;
				y = t_y;
				matrix[y][x] = c;
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
		matrix
	}
}