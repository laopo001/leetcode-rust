struct Solution;
use std::collections::HashMap;
// Runtime: 0 ms, faster than 100.00%
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for i in nums {
            if let Some(v) = map.get_mut(&i) {
                *v += 1;
            } else {
                map.insert(i, 1);
            }
        }
        for (k, v) in map {
            if v == 1 {
                return k;
            }
        }
        return 0;
    }
}

#[test]
fn test() {
    assert_eq!(Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]), 9);
}
