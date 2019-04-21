use crate::base::Solution;

impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        if n % 2 == 0 {
            return true;
        } else {
            return false;
        }
    }
}
