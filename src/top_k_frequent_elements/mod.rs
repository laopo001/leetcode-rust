use crate::base::Solution;
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut heap: BinaryHeap<(usize, i32)> = BinaryHeap::new();
        let mut map: HashMap<i32, usize> = HashMap::new();
        for item in nums {
            *map.entry(item).or_insert(0) += 1;
        }
        for (item, size) in map {
            heap.push((size, item));
        }
        let mut res: Vec<i32> = vec![];
        for _ in 0..k {
            // if heap.len() > (i as usize)  {
            let t = heap.pop().unwrap();
            res.push(t.1);
            // }
        }
        return res;
    }
}