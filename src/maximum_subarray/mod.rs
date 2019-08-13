#![allow(unused)]
use crate::base::Solution;
use std::cmp::max;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut res = std::i32::MIN;
        let mut sum = 0;
        for item in nums {
            sum = max(sum + item, item);
            res = max(res, sum);
        }
        return res;
    }
}
