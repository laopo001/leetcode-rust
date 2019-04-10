use crate::base::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut map: HashMap<u8, i32> = HashMap::new();
        for item in s.as_bytes() {
            if map.contains_key(&item) {
                let t = map.get_mut(&item).unwrap();
                *t += 1;
            } else {
                map.insert(*item, 1);
            }
        }
        for item in t.as_bytes() {
            if map.contains_key(&item) {
                let t = map.get_mut(&item).unwrap();
                *t -= 1;
                if *t < 0 {
                    return false;
                }
            } else {
                return false;
            }
        }
        for values in map {
            if values.1 != 0 {
                return false;
            }
        }
        return true;
    }
}
