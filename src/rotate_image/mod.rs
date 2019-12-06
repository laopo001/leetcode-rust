#![allow(dead_code, unused)]

struct Solution;

impl Solution {
    // Runtime: 0 ms, faster than 100.00%
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        for i in 0..m {
            if i >= m - i - 1 {
                break;
            }
            for j in 0..m {
                let t = matrix[i][j];
                matrix[i][j] = matrix[m - i - 1][j];
                matrix[m - i - 1][j] = t;
            }
        }
        // println!("{:?}",matrix);
        for i in 0..m {
            for j in 0..m {
                if j > i {
                    break;
                }
                let t = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = t;
            }
        }
    }
}
