struct Solution;

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut l_c = 0;
        let mut r_c = 0;
        let mut res = 0;
        for i in s.as_bytes() {
            if 'R' as u8 == *i {
                r_c += 1;
            }
            if 'L' as u8 == *i {
                l_c += 1;
            }
            if r_c > 0 && l_c == r_c {
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::balanced_string_split("RLRRLLRLRL".to_string()), 4);
    assert_eq!(Solution::balanced_string_split("RLLLLRRRLR".to_string()), 3);
    assert_eq!(Solution::balanced_string_split("LLLLRRRR".to_string()), 1);
}
