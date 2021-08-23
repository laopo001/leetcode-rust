struct Solution;
use std::collections::VecDeque;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut graph = vec![vec![]; num_courses as usize];
        let mut inc = vec![0; num_courses as usize];
        for x in prerequisites.iter() {
            graph[x[1] as usize].push(x[0]);
            inc[x[0] as usize] += 1;
        }
        let mut q: VecDeque<i32> = VecDeque::new();
        for i in 0..num_courses as usize {
            if (inc[i] == 0) {
                q.push_back(i as i32);
            }
        }
        dbg!(&graph, &inc, &q);
        while (!q.is_empty()) {
            let t = q.pop_front().unwrap();
            res.push(t);
    
            for a in graph[t as usize].iter() {
                inc[*a as usize] -= 1;
                if inc[*a as usize] == 0 {
                    q.push_back(*a);
                }
            }
        }
        if (res.len() != num_courses as usize) {
            res.clear();
        }
        return res;
    }
}

#[test]
fn test() {
    let mut arr = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
    let res = Solution::find_order(4, arr);
    // let mut arr = vec![
    //     vec![1, 0],
    //     vec![0, 3],
    //     vec![0, 2],
    //     vec![3, 2],
    //     vec![2, 5],
    //     vec![4, 5],
    //     vec![5, 6],
    //     vec![2, 4],
    // ];
    // let res = Solution::find_order(7, arr);
    // let mut arr = vec![
    //     vec![5, 8],
    //     vec![3, 5],
    //     vec![1, 9],
    //     vec![4, 5],
    //     vec![0, 2],
    //     vec![7, 8],
    //     vec![4, 9],
    // ];
    // let res = Solution::find_order(10, arr);

    dbg!(res);
}
