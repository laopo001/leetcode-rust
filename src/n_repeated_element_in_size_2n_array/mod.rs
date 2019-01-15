use std::collections::{HashMap, HashSet};

pub fn repeated_n_times(a: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, bool> = HashMap::new();
    for item in a {
        if (!map.contains_key(&item)) {
            map.insert(item, true);
        } else {
            return item;
        }
    }
    return 123;
}