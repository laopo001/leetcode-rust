use crate::base::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map: HashMap<String, String> = HashMap::new();
        let mut map2: HashMap<String, String> = HashMap::new();
        let len = s.len();
        if t.len() != len {
            return false;
        }
        for i in 0..len {
            let a = s.get(i..(i + 1)).unwrap();
            let b = t.get(i..(i + 1)).unwrap();
            if map.contains_key(a) {
                if map.get(a).unwrap() != b {
                    return false;
                }
            } else {
                if map2.contains_key(b) {
                    if map2.get(b).unwrap() != a {
                        return false;
                    }
                }
                map.insert(a.to_string(), b.to_string());
                map2.insert(b.to_string(), a.to_string());
            }
        }
        return true;
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::is_isomorphic("ab".to_string(), "aa".to_string()),
        false
    )
}
