struct Solution;

impl Solution {
    // 8 ms, faster than 100.00%
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len() as i32;
        if m == 0 {
            return grid;
        }
        let n = grid[0].len() as i32;
        let mut res = vec![];
        let len = m * n;
        for i in 0..m {
            let mut t = vec![];

            for j in 0..n {
                let mut z = i * n + j - k;
                while z < 0 {
                    z += len;
                }
                let y = ((z % len) / n);
                let x = (z % len % n); // or let x = (z % len % n);

                t.push(grid[y as usize][x as usize]);
            }
            res.push(t);
        }
        return res;
    }
}

#[test]
fn test() {
    // let mut arr = vec![
    //     vec![3, 8, 1, 9],
    //     vec![19, 7, 2, 5],
    //     vec![4, 6, 11, 10],
    //     vec![12, 0, 21, 13],
    // ];
    // let res = Solution::shift_grid(arr, 4);

    let mut arr = vec![
        vec![1],
        vec![2],
        vec![3],
        vec![4],
        vec![7],
        vec![6],
        vec![5],
    ];
    let res = Solution::shift_grid(arr, 23);

    dbg!(res);
}
