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
}