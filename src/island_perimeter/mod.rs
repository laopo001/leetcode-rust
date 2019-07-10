use crate::base::Solution;

fn get_one_nums(grid: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
	let mut res = 0;
	let x_len = grid.len();
    if x_len == 0 {
        return res;
    }
    let y_len = grid[0].len();
	if i!=0 && grid[i - 1][j] == 1 {
		res += 1;
	}
	if i !=x_len -1 && grid[i + 1][j] == 1 {
		res += 1;
	}
	if j!=0 && grid[i][j - 1] == 1 {
		res += 1;
	}
	if j !=y_len -1 && grid[i][j + 1] == 1 {
		res += 1;
	}
	res
}

impl Solution {
	pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
		let mut res = 0;
		for i in 0..grid.len() {
			for j in 0..grid[i].len() {
				if grid[i][j] == 1 {
					res += 4 - get_one_nums(&grid, i, j);
				}
			}
		}
		res
	}
}