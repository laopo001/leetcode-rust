use crate::base::Solution;

impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut v: Vec<i32> = vec![];
        let mut res = 0;
        for s in ops {
            let p = s.parse::<i32>();
            if p.is_err() {
                if s == "C" {
                    let num = v.pop().unwrap();
                    res -= num;
                }
                if s == "D" {
                    let num = v[v.len() - 1] * 2;
                    res += num;
                    v.push(num);
                }
                if s == "+" {
                    let num = v[v.len() - 1];
                    let num2 = v[v.len() - 2];
                    res += num + num2;
                    v.push(num + num2);
                }
            } else {
                let num = p.unwrap();
                v.push(num);
                res += num;
            }
        }
        return res;
    }
}
