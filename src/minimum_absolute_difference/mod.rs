struct Solution;

// Runtime: 12 ms, faster than 100.00%
impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        if arr.len() <= 1 {
            return vec![];
        }
        arr.sort();
        let mut min = std::i32::MAX;
        let mut res: Vec<Vec<i32>> = vec![];
        for i in 0..(arr.len() - 1) {
            let temp = arr[i + 1] - arr[i];
            if temp < min {
                min = temp;
                res.clear();
                res.push(vec![arr[i], arr[i + 1]]);
            } else if temp == min {
                res.push(vec![arr[i], arr[i + 1]]);
            }
        }
        res
    }
}
