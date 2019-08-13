#![allow(unused)]
use crate::base::Solution;

impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut res: i32 = 0;
        for i in 0..grid.len() {
            let row = &grid[i];
            for j in 0..row.len() {
                let item = row[j];
                res += if item == 0 { 0 } else { item * 4 + 2 };
                match grid.get(i + 1) {
                    Some(v) => {
                        res -= Solution::min(v[j], item) * 2;
                    }
                    None => {}
                };
                match grid[i].get(j + 1) {
                    Some(v) => {
                        res -= Solution::min(*v, item) * 2;
                    }
                    None => {}
                };
            }
        }
        return res;
    }
    fn min(l: i32, r: i32) -> i32 {
        if l > r {
            r
        } else {
            l
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::surface_area(vec![vec![2]]), 10)
}
