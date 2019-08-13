use crate::base::Solution;

impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut res = 0;
        for i in 0..len {
            for j in (i + 1)..len {
                res += (nums[i] ^ nums[j]).count_ones();
            }
        }
        return res as i32;
    }
}
