use crate::base::Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut res = "".to_string();
        let mut count = 0;
        for i in s.as_bytes() {
            if *i == b'(' {
                if count != 0 {
                    res += &std::str::from_utf8(&[*i]).unwrap();
                }
                count += 1;
            }
            if *i == b')' {
                count -= 1;
                if count != 0 {
                    res += &std::str::from_utf8(&[*i]).unwrap();
                }
            }
        }
        return res;
    }
}
