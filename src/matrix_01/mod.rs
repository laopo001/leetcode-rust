struct Solution;

//  24 ms, faster than 90.91%
impl Solution {
    pub fn update_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = matrix.len();
        if (m == 0) {
            panic!("error")
        }
        let n = matrix[0].len();
        let mut res = vec![vec![std::i16::MAX as i32; n]; m];
        for i in 0..m {
            for j in 0..n {
                if (matrix[i][j] == 0) {
                    res[i][j] = 0;
                    continue;
                }
                if (i as i32 - 1 >= 0 && matrix[i - 1][j] == 0) {
                    res[i][j] = 1;
                    continue;
                }
                if (j as i32 - 1 >= 0 && matrix[i][j - 1] == 0) {
                    res[i][j] = 1;
                    continue;
                }
                if (i as i32 - 1 >= 0 && matrix[i - 1][j] == 1) {
                    res[i][j] = std::cmp::min(res[i][j], res[i - 1][j] + 1);
                }
                if (j as i32 - 1 >= 0 && matrix[i][j - 1] == 1) {
                    res[i][j] = std::cmp::min(res[i][j], res[i][j - 1] + 1);
                }
            }
        }
        for i in 0..m {
            let i = m - 1 - i;
            for j in 0..n {
                let j = n - 1 - j;
                if (matrix[i][j] == 0) {
                    res[i][j] = 0;
                    continue;
                }
                if (i + 1 < m && matrix[i + 1][j] == 0) {
                    res[i][j] = 1;
                    continue;
                }
                if (j + 1 < n && matrix[i][j + 1] == 0) {
                    res[i][j] = 1;
                    continue;
                }
                if (i + 1 < m && matrix[i + 1][j] == 1) {
                    res[i][j] = std::cmp::min(res[i][j], res[i + 1][j] + 1);
                }
                if (j + 1 < n && matrix[i][j + 1] == 1) {
                    res[i][j] = std::cmp::min(res[i][j], res[i][j + 1] + 1);
                }
            }
        }
        res
    }
}
