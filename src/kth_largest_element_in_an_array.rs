struct Solution;

impl Solution {
    // 0 ms, faster than 100.00%
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        return nums[nums.len()  - k as usize];
    }
}