#![allow(unused)]
struct Solution;
// Runtime: 0 ms, faster than 100.00%
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        for item in intervals.iter_mut() {
            let mut b = true;
            let mut i = 0;
            let mut q = vec![];
            let mut c = 0;
            for x in res.iter_mut() {
                i += 1;
                if item[0] > x[1] || item[1] < x[0] {
                    // b = true;
                } else {
                    let min = std::cmp::min(item[0], x[0]);
                    let max = std::cmp::max(item[1], x[1]);
                    x[0] = min;
                    item[0] = min;
                    x[1] = max;
                    item[1] = max;
                    q.push(i);
                    b = false;
                }
            }
            // println!("{:?},{:?},{:?}", q.len(), res, b);
            if q.len() > 1 {
                for mut i in q.into_iter().rev().skip(1) {
                    i -= 1;
                    res.splice(i..(i + 1), vec![].iter().cloned());
                }
            }
            if b {
                res.push(item.clone());
            }
        }
        res
    }
}
