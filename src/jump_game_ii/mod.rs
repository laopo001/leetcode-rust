#![allow(unused)]
struct Solution;
// Runtime: 0 ms, faster than 100.00%
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len <= 1 {
            return 0;
        }
        let mut count = 0;
        let mut max_index = 0;
        loop {
            // println!("{:?}", max_index);
            let start = max_index + nums[max_index] as usize;
            count += 1;
            if start >= len - 1 {
                break;
            }
            let mut max_i32 = 0;
            let temp = max_index;
            for i in max_index..=start {
                if max_i32 < nums[i] + (i - temp) as i32 {
                    max_i32 = nums[i] + (i - temp) as i32;
                    max_index = i;
                }
            }
        }
        count
    }
}
#[test]
fn test() {
    Solution::jump(vec![4, 1, 1, 3, 1, 1, 1]);
}
