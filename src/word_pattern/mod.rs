use crate::base::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, str: String) -> bool {
        let mut map: HashMap<char, &str> = HashMap::new();
        let mut map2: HashMap<&str, char> = HashMap::new();
        let res = true;
        let len = pattern.len();
        let arr: Vec<&str> = str.split(' ').collect();
        if len != arr.len() {
            return false;
        }
        let chars: Vec<char> = pattern.chars().collect();
        for i in 0..len {
            if !map.contains_key(&chars[i]) {
                map.insert(chars[i], arr[i]);
            } else {
                if *map.get(&chars[i]).unwrap() != arr[i] {
                    return false;
                }
            }
            if !map2.contains_key(&arr[i]) {
                map2.insert(arr[i], chars[i]);
            } else {
                if *map2.get(&arr[i]).unwrap() != chars[i] {
                    return false;
                }
            }
        }
        res
    }
}