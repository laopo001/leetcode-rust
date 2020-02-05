struct Solution;
use std::collections::HashMap;
// Runtime: 0 ms, faster than 100.00%
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for i in nums {
            if let Some(v) = map.get_mut(&i) {
                *v += 1;
            } else {
                map.insert(i, 1);
            }
        }
        let mut res = vec![];
        for (k, v) in map {
            if v == 1 {
                res.push(k);
            }
        }
        return res;
    }
}

#[test]
fn test() {
    let mut arr = Solution::single_number(vec![1, 2, 1, 3, 2, 5]);
    arr.sort();
    assert_eq!(arr, [3, 5]);
}
