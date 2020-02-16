struct Solution;
use std::collections::HashMap;
// Runtime: 0 ms, faster than 100.00%
impl Solution {
    pub fn custom_sort_string(s: String, t: String) -> String {
        let s_bytes = s.as_bytes();
        let mut arr = [false; 26];
        for i in s_bytes {
            arr[(*i - ('a' as u8)) as usize] = true;
        }
        let mut res1: Vec<u8> = vec![];
        let mut res: Vec<u8> = vec![];
        let mut map: HashMap<u8, u32> = HashMap::new();
        let t_bytes = t.as_bytes();
        for i in t_bytes {
            if !arr[(*i - ('a' as u8)) as usize] {
                res.push(*i);
            } else {
                if let Some(v) = map.get_mut(i) {
                    *v += 1;
                } else {
                    map.insert(*i, 1);
                }
            }
        }
        for i in s_bytes {
            if let Some(v) = map.get(i) {
                for _ in 0..(*v as usize) {
                    res1.push(*i);
                    // dbg!(String::from_utf8(res1.clone()).unwrap());
                }
            }
        }
        String::from_utf8(res1).unwrap() + String::from_utf8(res).unwrap().as_str()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::custom_sort_string("kqep".to_string(), "pekeq".to_string()),
        "kqeep".to_string()
    );
    assert_eq!(
        Solution::custom_sort_string("cbafg".to_string(), "abcd".to_string()),
        "cbad".to_string()
    );
    assert_eq!(
        Solution::custom_sort_string("cba".to_string(), "abcd".to_string()),
        "cbad".to_string()
    );
}
