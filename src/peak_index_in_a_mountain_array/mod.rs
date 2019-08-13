use crate::base::Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(a: Vec<i32>) -> i32 {
        let mut max_vale = i32::min_value();
        let mut index = 0;
        for i in 0..(a.len()) {
            if a[i] > max_vale {
                max_vale = a[i];
                index = i as i32;
            }
        }
        return index;
    }
}