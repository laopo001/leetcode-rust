use crate::base::Solution;

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let d = 10;
        for i in left..=right {
            let mut n = i;
            let mut b = false;
            while n != 0 {
                let t = n % d;
                if t == 0 {
                    b = true;
                    break;
                }
                if i % t != 0 {
                    b = true;
                    break;
                }
                n = (n - t) / d;
            }
            if !b {
                res.push(i);
            }
        }
        return res;
    }
}
