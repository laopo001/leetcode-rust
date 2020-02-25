#![feature(drain_filter)]
struct Solution;
// Runtime: 0 ms, faster than 100.00%
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut res = "".to_string();
        let mut arr = s.split(' ').collect::<Vec<&str>>();
        arr.retain(|x| *x != "");
        if arr.len() == 0 {
            return res;
        }
        arr.reverse();
        for item in arr {
            res = res + " " + item;
        }
        return res.split_off(1);
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::reverse_words("a good  example".to_string()),
        "example good a".to_string()
    );
    assert_eq!(Solution::reverse_words("".to_string()), "".to_string());
}
