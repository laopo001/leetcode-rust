#![allow(unused)]
use crate::base::Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut res =  "1".to_string();
        if n == 1 {
            return res;
        }
        for i in 1..n {
            res = Solution::say(res);
        }
        return res;
    }
    pub fn say(s: String) -> String {
        let mut i = 1;
        let mut j: char = 'q';
        let mut res = "".to_string();
        for c in s.chars() {
            if c == j {
                i = i + 1;
            } else {
                if j == 'q' {
                    j = c;
                } else {
                    res = res + i.to_string().as_str() + j.to_string().as_str();
                    j = c;
                    i = 1;
                }
            }
        }
        res = res + i.to_string().as_str() + j.to_string().as_str();
        return res;
    }
}

#[test]
fn test() {
    assert_eq!(Solution::say("11".to_string()), "21".to_string());
}
