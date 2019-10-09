use crate::base::Solution;

impl Solution {
    pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
        let mut q = 1;
        if dividend < 0 {
            dividend = -dividend;
            q = q * -1;
        }
        if divisor < 0 {
            divisor = -divisor;
            q = q * -1;
        }
        let mut i = 1;
        loop {
            if (i * divisor) > dividend {
                i = i / 2;
                break;
            } else {
                i = i * 2;
            }
        }
        loop {
            if (i * divisor) > dividend {
                i = i - 1;
                break;
            } else {
                if (i * divisor) == dividend {
                    return q * i;
                }
                i = i + 1;
            }
        }
        q * i
    }
}