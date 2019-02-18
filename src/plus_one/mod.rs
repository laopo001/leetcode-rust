#![allow(unused)]
use crate::base::Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let len = digits.len();
        let mut b = false;
        for i in 0..len {
            let index = len - 1 - i;
            if (digits[index] == 9) {
                digits[index] = 0;
            } else {
                digits[index] += 1;
                b = true;
                break;
            }
        }
        if (!b) {
            digits.insert(0, 1);
        }
        return digits;
    }
}
