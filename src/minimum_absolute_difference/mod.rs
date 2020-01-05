struct Solution;

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        if arr.len() <= 1 {
            return vec![];
        }
        arr.sort();
        let mut min = std::i32::MAX;
        let mut res: Vec<Vec<i32>> = vec![];
        for i in 0..(arr.len() - 1) {
            if arr[i + 1] - arr[i] < min {
                min = arr[i + 1] - arr[i];
                res.clear();
                res.push(vec![arr[i], arr[i + 1]]);
            } else if arr[i + 1] - arr[i] == min {
                res.push(vec![arr[i], arr[i + 1]]);
            }
        }
        res
    }
}
