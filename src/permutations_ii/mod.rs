struct Solution;
use std::collections::HashMap;

fn run(nums: Vec<i32>, num: i32) -> Vec<Vec<i32>> {
    println!("{:?}",nums);
    let mut res: Vec<Vec<i32>> = vec![];
    let slice = nums.as_slice();
    let mut t:Option<i32> = None;
    for i in 0..slice.len() {
        if t.is_some() {
            if  t.unwrap() == slice[i] {
                continue;
            }else{
                t= None;
            }
        }else{
            if slice[i] == num {
                t = Some(slice[i]);
            }

        }
        let (a, b) = slice.split_at(i);
        let mut temp = a.to_vec();
        temp.extend_from_slice(&[num]);
        temp.extend_from_slice(b);
        res.push(temp);
    }
    if t.is_none() || num != t.unwrap(){
        let mut w = nums.clone();
        w.push(num);
        res.push(w);
    }
    // println!("{:?}", res);
    return res;
}

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 1 {
            return vec![vec![nums[0]]];
        }
        let mut nums=nums;
        nums.sort();
        let mut res: Vec<Vec<i32>> = vec![vec![nums[0]]];
        for i in 1..nums.len() {
            let mut temp: Vec<Vec<i32>> = vec![];
            for arr in res {
                let i = run(arr, nums[i]);
                for q in i {
                    temp.push(q);
                }
            }
            // println!("{:?}", temp);
            res = temp;
        }
        let mut map: HashMap<String, bool> = HashMap::new();
        let mut q: Vec<Vec<i32>> = vec![];
        for item in res {
            let s = item.iter().map(|x| { x.to_string() }).collect::<Vec<String>>().join("").to_string();
            if !map.get(&s).unwrap_or(&false) {
                map.insert(s, true);
                q.push(item);
            }
        }
        return q;
    }
}