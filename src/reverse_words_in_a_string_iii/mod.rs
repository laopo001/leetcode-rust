use crate::base::Solution;

impl Solution {
    // #![feature(split_ascii_whitespace)]
    pub fn reverse_words(s: String) -> String {
        let mut res = "".to_string();
        let mut b = false;
        unsafe {
            for item in s.split(' ') {
                let mut z = item.to_string();
                let temp = z.as_mut_vec();
                temp.reverse();
                if b {
                    res += " ";
                }
                b = true;
                res += z.as_str();
            }
        }
        return res;
    }
}
