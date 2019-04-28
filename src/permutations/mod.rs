use crate::base::Solution;

fn run(nums: Vec<i32>, num: i32) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];
    let slice = nums.as_slice();
    for i in 0..slice.len() {
        let (a, b) = slice.split_at(i);
        let mut temp = a.to_vec();
        temp.extend_from_slice(&[num]);
        temp.extend_from_slice(b);
        res.push(temp);
    }
    let mut w = nums.clone();
    w.push(num);
    res.push(w);
    // println!("{:?}", res);
    return res;
}

impl Solution {
    // 100%
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 1 {
            return vec![vec![nums[0]]];
        }
        let mut res: Vec<Vec<i32>> = vec![vec![nums[0]]];
        for i in 1..nums.len() {
            let mut temp: Vec<Vec<i32>> = vec![];
            for arr in res {
                let i = run(arr, nums[i]);
                for q in i {
                    temp.push(q);
                }
            }
            res = temp;
        }
        return res;
    }
}