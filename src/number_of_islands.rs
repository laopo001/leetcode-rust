struct Solution;

fn find(grid: &Vec<Vec<char>>, arr: &mut Vec<Vec<i32>>, i: usize, j: usize) {
    if (*grid.get(i - 1).unwrap_or(&vec![]).get(j).unwrap_or(&'0') == '1' && arr[i - 1][j] == 0) {
        arr[i - 1][j] = 1;
        find(grid, arr, i - 1, j);
    }
    if (*grid.get(i + 1).unwrap_or(&vec![]).get(j).unwrap_or(&'0') == '1' && arr[i + 1][j] == 0) {
        arr[i + 1][j] = 1;
        find(grid, arr, i + 1, j);
    }
    if (*grid.get(i).unwrap_or(&vec![]).get(j - 1).unwrap_or(&'0') == '1' && arr[i][j - 1] == 0) {
        arr[i][j - 1] = 1;
        find(grid, arr, i, j - 1);
    }
    if (*grid.get(i).unwrap_or(&vec![]).get(j + 1).unwrap_or(&'0') == '1' && arr[i][j + 1] == 0) {
        arr[i][j + 1] = 1;
        find(grid, arr, i, j + 1);
    }
}

impl Solution {
    // 4 ms, faster than 96.30%
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        let m = grid.len();
        if (m == 0) {
            return 0;
        }
        let n = grid[0].len();
        let mut arr = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                if (grid[i][j] == '1' && arr[i][j] == 0) {
                    arr[i][j] = 1;
                    find(&grid, &mut arr, i, j);
                    count += 1;
                }
            }
        }
        return count;
    }
}
