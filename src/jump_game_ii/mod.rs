#![allow(unused)]
struct Solution;

fn run(nums: &Vec<i32>, i: usize) -> i64 {
    // println!("{:?}", i);
    let len = nums.len();
    if i >= len - 1 {
        return 0;
    }
    if nums[i] == 0 {
        return 1 + run(&nums, i + 1);
    }
    std::cmp::min(
        1 + if i + nums[i] as usize > len - 1 {
            0
        } else {
            run(&nums, i + nums[i] as usize)
        },
        1 + run(&nums, i + 1),
    )
}
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        run(&nums, 0) as i32
    }
}
#[test]
fn test() {
    Solution::jump(vec![4, 1, 1, 3, 1, 1, 1]);
}
