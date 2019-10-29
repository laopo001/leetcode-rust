use crate::base::Solution;

impl Solution {
    pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
        if dividend == -2147483648 && divisor == -1 {
            return 2147483647;
        }
        let mut dividend = dividend as i64;
        let mut divisor = divisor as i64;
        if divisor == 1 {
            return dividend as i32;
        }
        if divisor == -1 {
            return -dividend as i32;
        }
        let mut q = 1 as i64;
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
                    return (q * i) as i32;
                }
                i = i + 1;
            }
        }
        (q * i) as i32
    }
}
