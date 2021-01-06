struct Solution;
use std::collections::VecDeque;

fn vec_max_value(nums: &VecDeque<i32>) -> i32 {
    let mut max_value = i32::min_value();
    for i in 0..nums.len() {
        if nums[i] > max_value {
            max_value = nums[i];
        }
    }
    max_value
}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return nums;
        }
        let mut max_value = i32::min_value();
        let mut queue = VecDeque::new();
        let mut res: Vec<i32> = vec![];
        for i in 0..(k as usize) {
            queue.push_back(nums[i]);
            if nums[i] > max_value {
                max_value = nums[i];
            }
        }
        res.push(max_value);
        for i in (k as usize)..nums.len() {
            queue.push_back(nums[i]);
            let front = queue.pop_front().unwrap();
            // println!("{}",front);
            if front == max_value {
                max_value = vec_max_value(&queue);
                res.push(max_value);
            } else if nums[i] > max_value {
                max_value = nums[i];
                res.push(max_value);
            } else {
                res.push(max_value);
            }
        }
        return res;
    }
}