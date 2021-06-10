struct Solution;

impl Solution {
    // 28 ms, faster than 12.50%
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut res = vec![];
        for i in 1..=n {
            if (i == 1) {
                res.push(vec![0]);
                res.push(vec![1]);
            } else {
                let mut q = vec![];
                for x in res.clone() {
                    let mut a = x;
                    a.push(0);
                    q.push(a);
                }
                res.reverse();
                for x in res {
                    let mut a = x;
                    a.push(1);
                    q.push(a);
                }
                res = q;
            }
        }
        // dbg!(&res);
        res.into_iter()
            .map(|mut x| {
                let mut s = 0;
                let mut i = 0;
                // x.reverse();
                for n in x {
                    s += n * 2_i32.pow(i);
                    i += 1;
                }
                return s;
            })
            .collect()
    }
    // 28 ms, faster than 12.50%
    pub fn gray_code2(n: i32) -> Vec<i32> {
        let mut res = vec![vec![0; n as usize]; 2_usize.pow(n as u32)];
        for i in 0..(n as usize) {
            if (i == 0) {
                res[1][i] = 1;
            } else {
                let len = 2_usize.pow((i) as u32);
                for j in 0..(i) {
                    for k in 0..len {
                        res[len + k][j] = res[len - 1 - k][j];
                    }
                }
                for k in 0..len {
                    res[k][i] = 0;
                }
                for k in len..(2 * len) {
                    res[k][i] = 1;
                }
            }
        }
        // dbg!(&res);
        res.into_iter()
            .map(|mut x| {
                let mut s = 0;
                let mut i = 0;
                // x.reverse();
                for n in x {
                    s += n * 2_i32.pow(i);
                    i += 1;
                }
                return s;
            })
            .collect()
    }
    // 4 ms, faster than 100.00%
    pub fn gray_code3(n: i32) -> Vec<i32> {
        let mut ans = vec![0, 1];
        for i in 2..=n {
            let num = 2_i32.pow((i as u32) - 1);
            for j in (0..ans.len()).rev() {
                ans.push(num + ans[j]);
            }
        }
        return ans;
    }
}

#[test]
fn test() {
    assert_eq!(Solution::gray_code3(3), vec![0, 1, 3, 2, 6, 7, 5, 4]);
    assert_eq!(Solution::gray_code(3), vec![0, 1, 3, 2, 6, 7, 5, 4]);
    assert_eq!(Solution::gray_code2(3), vec![0, 1, 3, 2, 6, 7, 5, 4]);
    assert_eq!(Solution::gray_code(2), vec![0, 1, 3, 2]);
    assert_eq!(Solution::gray_code2(2), vec![0, 1, 3, 2]);
}
