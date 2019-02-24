#![allow(unused)]
use crate::base::{Solution, TreeNode};

pub fn to_decimal(n: i32, d: i32) -> String {
    let mut res = "".to_string();
    let mut n = n;
    while n != 0 {
        let t = n % d;
        res = t.to_string() + res.as_str();
        n = (n - t) / d;
    }
    return res.to_string();
}

pub fn string_to_num(s: String, d: i32) -> i32 {
    let mut res = 0;
    let bytes = s.as_bytes();
    for i in 0..bytes.len() {
        let index = bytes.len() - 1 - i;
        res += (bytes[i] as i32 - 48) * d.pow(index as u32);
    }
    return res;
}

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut s = to_decimal(num, 2);
        let mut temp = String::new();

        for c in s.bytes() {
            if c == b'0' {
                temp.push('1');
            } else {
                temp.push('0');
            }
        }
        return string_to_num(temp, 2);
    }
}
