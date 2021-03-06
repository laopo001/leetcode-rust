struct Solution;

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        return if b == 0 {
            a
        } else {
            Solution::get_sum(a ^ b, (a & b) << 1)
        };
    }
}
