struct Solution;
use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut queue: VecDeque<u8> = VecDeque::new();
        let mut map: HashMap<u8, u8> = HashMap::new();
        map.insert(b'(', b')');
        map.insert(b'{', b'}');
        map.insert(b'[', b']');
        for i in s.as_bytes() {
            if *i == b'(' || *i == b'{' || *i == b'[' {
                queue.push_back(*i);
            } else {
                if queue.len() == 0 {
                    return false;
                } else {
                    let c = queue.pop_back().unwrap();
                    if map[&c] != *i {
                        return false;
                    }
                }
            }
        }
        queue.len() == 0
    }
}