#![allow(unused)]

fn run(nums: &Vec<i32>, i: usize) -> i32 {
    if i >= nums.len() {
        return 0;
    }
    std::cmp::min(
        1 + run(&nums, i + nums[i] as usize),
        2 + run(&nums, i + nums[i + 1] as usize),
    )
}

struct Solution;
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        run(&nums, 0)
    }
}

#[test]
fn test() {
    Solution::jump(vec![2, 3, 1, 1, 4]);
}
