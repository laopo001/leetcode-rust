use crate::base::Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        if m == 0 {
            return;
        }
        let n = matrix[0].len();
        let mut copy = vec![vec![1; n]; m];
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    for i in 0..m {
                        copy[i][j] = 0;
                    }
                    for j in 0..n {
                        copy[i][j] = 0;
                    }
                }
            }
        }

        for i in 0..m {
            for j in 0..n {
                matrix[i][j] = matrix[i][j] * copy[i][j];
            }
        }
    }
}
