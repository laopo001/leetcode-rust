struct Solution;
use std::collections::HashMap;

fn run(
    r: i32,
    c: i32,
    a: i32,
    b: i32,
    i: i32,
    j: i32,
    arr: &mut Vec<Vec<bool>>,
    index: i32,
    map: &mut HashMap<i32, Vec<Vec<i32>>>,
) {
    if i < r && i >= 0 && j < c && j >= 0 {
        if arr[i as usize][j as usize] {
            return;
        }
        arr[i as usize][j as usize] = true;
        if let Some(v) = map.get_mut(&index) {
            v.push(vec![i, j]);
        } else {
            map.insert(index, vec![vec![i, j]]);
        }
        run(r, c, a, b, i + a, j, arr, index + 1, map);
        run(r, c, a, b, i, j + b, arr, index + 1, map);
    }
}
// Runtime: 16 ms, faster than 100.00%
impl Solution {
    pub fn all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let mut arr: Vec<Vec<bool>> = vec![vec![false; c as usize]; r as usize];
        let mut res = vec![vec![r0, c0]];
        arr[r0 as usize][c0 as usize] = true;
        let mut map: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();
        let i = 0;
        run(r, c, 1, -1, r0 + 1, c0, &mut arr, i + 1, &mut map);
        run(r, c, -1, 1, r0 - 1, c0, &mut arr, i + 1, &mut map);
        run(r, c, 1, 1, r0, c0 + 1, &mut arr, i + 1, &mut map);
        run(r, c, -1, -1, r0, c0 - 1, &mut arr, i + 1, &mut map);
        let mut temp: Vec<(i32, Vec<Vec<i32>>)> = map.into_iter().collect();
        temp.sort_by(|a, b| {
            return a.cmp(b);
        });
        for a in temp {
            for b in a.1 {
                res.push(b);
            }
        }
        return res;
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::all_cells_dist_order(2, 2, 0, 1),
        vec![vec![0, 1], vec![1, 1], vec![0, 0], vec![1, 0]]
    );
}
