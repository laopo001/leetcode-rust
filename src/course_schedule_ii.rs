use std::usize;

struct Solution;

fn is_all_zero(arr: &Vec<i32>) -> bool {
    for i in arr {
        if *i != 0 {
            return false;
        }
    }
    return true;
}

fn run(start: i32, matrix: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut res = vec![];
    res.push(start);
    for (i, arr) in matrix.iter().enumerate() {
        if (arr[start as usize] != 0) {}
    }
    return res;
}

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut matrix = vec![vec![0; num_courses as usize]; num_courses as usize];
        for i in prerequisites.iter() {
            matrix[i[0] as usize][i[1] as usize] = 1;
        }
        // dbg!(&matrix);

        for (i, arr) in matrix.iter().enumerate() {
            if (is_all_zero(arr)) {}
        }
        return vec![];
    }
}

#[test]
fn test() {
    let mut arr = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
    Solution::find_order(4, arr);
}
