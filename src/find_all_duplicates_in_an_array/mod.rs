use crate::base::Solution;
use std::collections::HashMap;
impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let mut map: HashMap<i32, i32> = HashMap::new();
        for item in nums {
            if map.contains_key(&item) {
                // let t = map.get_mut(&item).unwrap();
                // *t += 1;
                res.push(item);
            } else {
                map.insert(item, 1);
            }
        }
        return res;
    }
}
