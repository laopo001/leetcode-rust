use crate::base::Solution;

#[allow(dead_code)]
fn cmp(a: &str) -> bool {
    let len = a.len() / 2;
    for i in 0..(len) {
        if a.chars().nth(i) != a.chars().nth(a.len() - 1 - i) {
            return false;
        }
    }
    return true;
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len();
        let mut max_len = "";
        for i in 1..len {
            for j in 0..(i + 1) {
                if j > i / 2 {
                    continue;
                }
                let z = s.get(j..(i + 1)).unwrap();
                if cmp(z) {
                    if max_len.len() < z.len() {
                        max_len = z;
                    }
                }
            }
        }
        return max_len.to_string();
    }
}
