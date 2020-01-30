struct Solution;
use std::collections::HashMap;
// Runtime: 4 ms, faster than 100.00%
impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut map: HashMap<String, i32> = HashMap::new();
        for item in cpdomains {
            let split = item.split(" ");
            let temp = item.split(" ").collect::<Vec<&str>>();
            let [num, domain] = [temp[0], temp[1]];
            let num = num.to_string().parse::<i32>().unwrap();
            let strs: Vec<&str> = domain.clone().split(".").collect();
            let mut bb = false;
            strs.iter().rev().fold("".to_string(), |a, b| {
                let t = if !bb {
                    bb = true;
                    b.to_string() + a.as_str()
                } else {
                    b.to_string() + "." + a.as_str()
                };
                if let Some(v) = map.get_mut(&t) {
                    *v += num;
                } else {
                    map.insert(t.clone(), num);
                }
                return t;
            });
        }
        let mut res: Vec<String> = vec![];
        for (k, v) in map {
            res.push(v.to_string() + " " + k.as_str());
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::subdomain_visits(vec!["9001 discuss.leetcode.com".to_string()]),
        vec![
            "9001 discuss.leetcode.com".to_string(),
            "9001 leetcode.com".to_string(),
            "9001 com".to_string()
        ]
    );
}
