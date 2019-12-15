#![allow(unused)]
struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        for item in intervals {
            let mut b = false;
            for x in res.iter_mut() {
                if item[0] > x[1] || item[1] < x[0] {
                    b = true;
                    continue;
                }
                if item[0] <= x[0] && item[1] >= x[1] {
                    x[0] = item[0];
                    x[1] = item[1];
                }
                if item[0] >= x[0] && item[1] > x[1] {
                    x[1] = item[1];
                }
                if item[0] < x[0] && item[1] <= x[1] {
                    x[0] = item[0];
                }
            }
            if b {
                res.push(item);
            }
        }
        res
    }
}
