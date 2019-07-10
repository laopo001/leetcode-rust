use crate::base::Solution;

fn get_one_nums(grid: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
	let mut res = 0;
	let arr: Vec<i32> = vec![];
	if grid[i - 1][j] == 1 {
		res + 1;
	}
	if grid[i + 1][j] == 1 {
		res + 1;
	}
	if grid[i][j - 1] == 1 {
		res + 1;
	}
	if grid[i][j + 1] == 1 {
		res + 1;
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