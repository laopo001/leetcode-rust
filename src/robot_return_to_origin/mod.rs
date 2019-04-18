#![allow(unused)]
use crate::base::{Solution, TreeNode};
use std::collections::HashMap;

impl Solution {
    #[allow(mutable_borrow_reservation_conflict)]
    pub fn judge_circle(moves: String) -> bool {
        let mut map: HashMap<char, u32> = HashMap::new();
        map.insert('U', 0);
        map.insert('D', 0);
        map.insert('L', 0);
        map.insert('R', 0);
        for c in moves.chars() {
            let count = map.get(&c).unwrap();
            map.insert(c, count + 1);
        }
        return map.get(&'U').unwrap() == map.get(&'D').unwrap()
            && map.get(&'R').unwrap() == map.get(&'L').unwrap();
    }
}

#[test]
fn test() {
    assert_eq!(Solution::judge_circle("UD".to_string()), true)
}
