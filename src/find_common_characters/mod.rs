use crate::base::Solution;
use std::collections::HashMap;


impl Solution {
    #[allow(mutable_borrow_reservation_conflict)]
    pub fn common_chars(a: Vec<String>) -> Vec<String> {
        if a.len() == 0 {
            return a;
        }
        let mut a = a;
        let mut map: HashMap<char, usize> = HashMap::new();
        let first = a.remove(0);
        for c in first.chars() {
            let t = map.get(&c);
            if t != None {
                map.insert(c, t.unwrap() + 1);
            } else {
                map.insert(c, 1);
            }
        }
        for item in a {
            let mut map2: HashMap<char, usize> = HashMap::new();
            for c in item.chars() {
                if map.contains_key(&c) {
                    let t = map2.get(&c);
                    if t != None {
                        map2.insert(c, t.unwrap() + 1);
                    } else {
                        map2.insert(c, 1);
                    }
                }
            }
            // println!("123");
            for (c, size) in map.iter_mut() {
                let size2 = map2.get(c).unwrap_or(&0);
                *size = std::cmp::min(*size, *size2)
            }
        }
        let mut res: Vec<String> = vec![];
        for (c, size) in map.iter() {
            for _ in 0..(*size) {
                res.push((*c).to_string());
            }
        }
        return res;
    }
    // 利用数组下标，读取更快
    pub fn common_chars2(a: Vec<String>) -> Vec<String> {
        if a.len() == 0 {
            return a;
        }
        let mut a = a;
        let mut map: Vec<usize> = vec![0; 26];
        let first = a.remove(0);
        for c in first.as_bytes() {
            let t = map[*c as usize - b'a' as usize];
            map[*c as usize - b'a' as usize] = t + 1;
        }

        for item in a {
            let mut map2: Vec<usize> = vec![0; 26];
            for c in item.as_bytes() {
                if map[*c as usize - b'a' as usize] != 0 {
                    let t = map2[*c as usize - b'a' as usize];
                    map2[*c as usize - b'a' as usize] = t + 1;
                }
            }
            for (i, size) in map.iter_mut().enumerate() {
                // let size2 = map2.get(c).unwrap_or(&0);
                let size2 = map2[i];
                *size = std::cmp::min(*size, size2)
            }
        }
        let mut res: Vec<String> = vec![];
        for (c, size) in map.iter().enumerate() {
            for _ in 0..(*size) {
                res.push(String::from_utf8(vec![c as u8 + b'a']).unwrap());
            }
        }
        return res;
    }
}
