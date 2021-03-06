use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.len() == 0 || s.get(0..1).unwrap() == "0" {
            return 0;
        }
        let len = s.len();
        let mut dp = vec![0; len + 1];
        dp[0] = 1;
        for i in 1..dp.len() {
            let a = s.get((i - 1)..i).unwrap().parse::<i32>().unwrap();
            if a == 0 {
                dp[i] = 0;
            } else {
                dp[i] = dp[i - 1];
            }
            if i > 1 {
                let b = s.get((i - 2)..(i - 1)).unwrap().parse::<i32>().unwrap();
                if b == 1 || (b == 2 && a <= 6) {
                    dp[i] += dp[i - 2];
                }
            }
        }
        return dp[dp.len() - 1];
    }
    pub fn num_decodings2(s: String) -> i32 {
        let arr = [1; 26];
        let mut t: Vec<Vec<i32>> = vec![];
        for i in 0..s.len() {
            let z = s.get(i..(i + 1)).unwrap().parse::<i32>().unwrap();
            if (t.len() == 0) {
                if z == 0 {
                    return 0;
                }
                t.push(vec![z]);
            } else {
                let mut c = 0;
                let mut removes = vec![];
                for (i, x) in t.iter_mut().enumerate() {
                    if x.len() == 1 {
                        let index = (x[0] * 10 + z - 1) as usize;
                        if arr.get(index).is_some() {
                            x.pop();
                            if z != 0 {
                                c += 1;
                            }
                        } else {
                            x.pop();
                            if z == 0 {
                                removes.push(i);
                            }
                        }
                    } else if x.len() == 0 {
                        if z == 0 {
                            removes.push(i);
                        }
                    } else {
                        panic!("a");
                    }
                    if z != 0 {
                        x.push(z);
                    }
                }
                t = t
                    .into_iter()
                    .enumerate()
                    .filter_map(|(i, x)| if !removes.contains(&i) { Some(x) } else { None })
                    .collect();
                for _ in 0..c {
                    t.push(vec![]);
                }
                if t.len() == 0 {
                    break;
                }
                // dbg!(&t);
            }
        }
        return t.len() as i32;
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::num_decodings("1001".to_string()), 0);
}

#[test]
fn test() {
    assert_eq!(Solution::num_decodings("0".to_string()), 0);
    assert_eq!(Solution::num_decodings("11".to_string()), 2);
    assert_eq!(Solution::num_decodings("111".to_string()), 3);
    assert_eq!(Solution::num_decodings("1111".to_string()), 5);
    assert_eq!(Solution::num_decodings("11111".to_string()), 8);
    assert_eq!(Solution::num_decodings("226".to_string()), 3);
    assert_eq!(Solution::num_decodings("227".to_string()), 2);
    assert_eq!(Solution::num_decodings("2274".to_string()), 2);
    assert_eq!(Solution::num_decodings("22712".to_string()), 4);
    assert_eq!(Solution::num_decodings("10".to_string()), 1);
    assert_eq!(Solution::num_decodings("01".to_string()), 0);
    assert_eq!(Solution::num_decodings("110".to_string()), 1);
    assert_eq!(Solution::num_decodings("100".to_string()), 0);
    assert_eq!(Solution::num_decodings("1010".to_string()), 1);
    assert_eq!(Solution::num_decodings("230".to_string()), 0);
    assert_eq!(Solution::num_decodings("301".to_string()), 0);
    assert_eq!(Solution::num_decodings("1001".to_string()), 0);
}
