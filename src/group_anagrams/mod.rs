use crate::base::Solution;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

fn format(s: String, res_map: &mut HashMap<String, Vec<String>>) {
    let mut map: HashMap<u8, usize> = HashMap::new();
    for c in s.bytes() {
        if map.contains_key(&c) {
            *map.get_mut(&c).unwrap() += 1;
        } else {
            map.insert(c, 1);
        }
    }
    let mut vec: Vec<(u8, usize)> = vec![];
    for c in map {
        vec.push((c.0, c.1));
    }
    vec.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut key = "".to_string();
    for (k, v) in vec {
        key = key + &k.to_string() + &v.to_string();
    }
    // let mut hasher = DefaultHasher::new();
    // vec.hash(&mut hasher);
    // let key = hasher.finish();
    if res_map.contains_key(&key) {
        let m = res_map.get_mut(&key).unwrap();
        m.push(s);
    } else {
        res_map.insert(key, vec![s]);
    }
}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for s in strs {
            format(s, &mut map);
        }
        let mut vec: Vec<Vec<String>> = vec![];
        for (_, value) in map {
            vec.push(value);
        }
        vec
    }
}
