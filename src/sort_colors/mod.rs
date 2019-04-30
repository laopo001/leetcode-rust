use crate::base::Solution;
// use std::collections::HashMap;

impl Solution {
    // 100%
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut count_0 = 0;
        let mut count_1 = 0;
        let mut count_2 = 0;
        for item in nums.iter() {
            if *item == 0 {
                count_0 += 1;
            } else if *item == 1 {
                count_1 += 1;
            } else if *item == 2 {
                count_2 += 1;
            }
        }
        for item in nums.iter_mut() {
            if count_0 > 0 {
                *item = 0;
                count_0 -= 1;
            } else if count_1 > 0 {
                *item = 1;
                count_1 -= 1;
            } else if count_2 > 0 {
                *item = 2;
                count_2 -= 1;
            }
        }
    }
}