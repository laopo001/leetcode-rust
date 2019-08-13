use crate::base::Solution;

impl Solution {
    pub fn smallest_range_i(a: Vec<i32>, k: i32) -> i32 {
        let mut max_v = i32::min_value();
        let mut min_v = i32::max_value();
        for item in a.into_iter() {
            if item > max_v {
                max_v = item;
            }
            if item < min_v {
                min_v = item;
            }
        }
        let res = max_v - min_v - 2 * k;
        if res < 0 {
            0
        } else {
            res
        }
    }
}
