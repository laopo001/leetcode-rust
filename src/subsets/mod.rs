#![allow(unused)]

struct Solution;

fn run(nums: &Vec<i32>, n: usize) -> Vec<Vec<i32>> {
    if n == 0 {
        return vec![vec![]];
    }

    let res = run(nums, n - 1);
    let mut temp: Vec<Vec<i32>> = vec![];
    temp.reserve(res.len() * 2);
    for x in res {
        let mut clone = x.clone();
        temp.push(x);
        clone.push(nums[n - 1]);
        temp.push(clone);
    }
    temp
}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        run(&nums, n)
    }
}
// []

// [1]

// [2]

// [1]  [2]
