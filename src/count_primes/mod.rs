use crate::base::Solution;

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

    pub fn count_primes(n: i32) -> i32 {
        if n <= 2 {
            return 0;
        }
        let mut res = 0;
        let mut arr: Vec<bool> = vec![true; (n + 1) as usize];
        arr[0] = false;
        arr[1] = false;
        // arr[2]= false;
        let mut prime = 2;
        for i in 2..(arr.len()) {
            if !arr[i] {
                continue;
            }
            // println!("{:?}1",arr);
            for j in 2..(arr.len()) {
                if j != prime && j % prime == 0 {
                    arr[j] = false;
                    // println!("{:?}",arr);
                }
            }
            let z = (|x| {
                for j in ((x + 1)..(arr.len())).rev() {
                    if arr[j] {
                        return j;
                    }
                }
                panic!("error");
                // return 0;
            })(i);
            if z < prime * prime {
                break;
            }
            prime = (|x| {
                for j in (x + 1)..(arr.len()) {
                    if arr[j] {
                        return j;
                    }
                }
                panic!("error");
                // return 0;
            })(i);
        }
        for i in 0..(arr.len()) {
            if arr[i] && i != n as usize {
                res += 1;
            }
        }
        return res;
    }
}
