use crate::base::Solution;

impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut res = 0;
        for chunk in nums.as_slice().chunks(2) {
            res += std::cmp::min(chunk[0], chunk[1]);
        }
        return res;
    }
}
