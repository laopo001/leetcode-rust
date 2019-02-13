#![allow(unused)]
use crate::base::Solution;

fn check(nums: &Vec<i32>, mid: i64, m: i32) -> bool {
    let mut c = 0;
    let mut sum: i64 = 0;
    for item in nums {
        if *item as i64 > mid {
            return false;
        }
        if sum + *item as i64 <= mid {
            sum = sum + *item as i64;
        } else {
            sum = *item as i64;
            c += 1;
        }
    }
    return c + 1 < m;
}

impl Solution {
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        let mut max: i64 = 0;
        let mut min: i64 = 0;
        let mut res: i64 = 0;
        for item in &nums {
            max += *item as i64;
        }
        while min <= max {
            let mut mid = (max + min) / 2;
            if check(&nums, mid, m) {
                max -= 1;
                res = mid;
            } else {
                min += 1;
            }
        }
        return res as i32;
    }
}
