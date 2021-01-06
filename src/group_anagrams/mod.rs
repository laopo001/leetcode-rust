struct Solution;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

fn format(s: String, res_map: &mut HashMap<u64, Vec<String>>) {
    let mut temp_s = s.clone();
    let vec = unsafe { temp_s.as_mut_vec() };
    vec.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    let mut hasher = DefaultHasher::new();
    vec.hash(&mut hasher);
    let key = hasher.finish();
    if res_map.contains_key(&key) {
        let m = res_map.get_mut(&key).unwrap();
        m.push(s);
    } else {
        res_map.insert(key, vec![s]);
    }
}

//Runtime: 12 ms, faster than 95.00%
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<u64, Vec<String>> = HashMap::new();
        for s in strs {
            format(s, &mut map);
        }
        map.drain().map(|(_, v)| v).collect()
    }
}
