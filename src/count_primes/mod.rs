struct Solution;

impl Solution {
    /// Time Limit Exceeded
    pub fn count_primes2(n: i32) -> i32 {
        let mut res = 0;
        for i in 2..n {
            let mut b = false;
            for j in 2..i {
                if i % j == 0 {
                    b = true;
                    break;
                }
            }
            if !b {
                res += 1;
            }
        }
        return res;
    }
    /// faster than 100.00%
    pub fn count_primes(n: i32) -> i32 {
        if n <= 2 {
            return 0;
        }
        let mut res = 0;
        let mut arr: Vec<bool> = vec![true; (n) as usize];
        arr[0] = false;
        arr[1] = false;
        'outer: for i in 2..(arr.len()) {
            if !arr[i] {
                continue;
            }
            // res += 1;
            let mut j = 2;
            while j * i < (n as usize) {
                arr[j * i] = false;
                j += 1;
            }
            for k in ((i + 1)..(arr.len())).rev() {
                if arr[k] {
                    if k < i * i {
                        break 'outer;
                    }
                    break;
                }
            }
        }
        for i in 0..(arr.len()) {
            if arr[i] {
                res += 1;
            }
        }
        return res;
    }

}
