struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut res = true;
        let mut res2 = true;
        let mut res3 = true;
        for (i,c) in word.as_bytes().iter().enumerate() {
            if *c < ('a' as u8) { // 大写
                if i > 0 {
                    res2 = false;
                }
                res3 = false;
            } else { // 小写
                res = false;
            }
        }
        return res || res2 || res3;
    }
}