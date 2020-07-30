struct Solution;
impl Solution {
    pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        if (nums.len() < 4) {
            return nums.iter().fold(1, |a, b| a * b);
        }
        nums.sort();
        let a = nums[nums.len() - 1];
        let q = nums[0] * nums[1];
        let w = nums[nums.len() - 2] * nums[nums.len() - 3];
        if (q > w) {
            return a * q;
        } else {
            return a * w;
        }
    }
}
