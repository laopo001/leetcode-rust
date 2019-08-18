use crate::base::Solution;

impl Solution {
    //  100 ms, faster than 50.00%
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let split = s.split("-");
        let vec: Vec<&str> = split.collect();
        let mut r = "".to_string();
        for item in vec {
            r += item;
        }
        let mut i = 0;
        let mut res = "".to_string();
        for item in r.chars().rev() {
            i += 1;
            res = item.to_string().to_ascii_uppercase() + &res;
            if k == i {
                res = "-".to_string() + &res;
                i = 0;
            }
        }
        if res.get(0..1).is_some() && res.get(0..1).unwrap() == "-" {
            res = res.get(1..).unwrap_or("").to_string();
        }
        res
    }
    // 8 ms, faster than 20.00%
    pub fn license_key_formatting2(s: String, k: i32) -> String {
        let split = s.split("-");
        let vec: Vec<&str> = split.collect();
        let mut r = "".to_string();
        for item in vec {
            r += item;
        }
        let mut i = 0;
        let mut res = "".to_string();
        for item in r.chars().rev() {
            i += 1;
            res = res + item.to_string().to_ascii_uppercase().as_str();
            if k == i {
                res = res + "-";
                i = 0;
            }
        }
        let start = res.len() - 1;
        let end = res.len();
        if res.get(start..end).is_some() && res.get(start..end).unwrap() == "-" {
            res = res.get(0..start).unwrap_or("").to_string();
        }
        unsafe {
            let temp = res.as_mut_vec();
            temp.reverse();
        }
        res
    }
}