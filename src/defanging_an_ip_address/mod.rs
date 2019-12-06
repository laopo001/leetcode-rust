use crate::base::Solution;

impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        let arr: Vec<&str> = address.split('.').collect();
        let mut res = "".to_string();
        for item in arr {
            if res == "".to_string() {
                res += item;
                continue;
            }
            res = res + "[.]" + item;
        }
        res
    }
}
