struct Solution;

fn run(mut a: u128, mut b: u128, num: &str, mut start: usize) -> bool {
    if (start == num.len()) {
        return false;
    }
    let mut res_b = true;
    loop {
        let c = (a + b);
        let res = c.to_string();
        for k in 0..res.len() {
            // dbg!(res[k..k + 1].to_string(), num[start + k..start + k + 1].to_string());
            if (start + k >= num.len()) {
                return false;
            }
            if (res[k..k + 1] != num[start + k..start + k + 1]) {
                res_b = false;
                break;
            }
        }
        if (!res_b) {
            break;
        } else {
            a = b;
            b = c;
            start = start + res.len();
            // dbg!(a, b, start);
            if (start == num.len()) {
                return true;
            }
        }
    }
    return res_b;
}

impl Solution {
    // 0 ms, faster than 100.00%
    pub fn is_additive_number(num: String) -> bool {
        let len = num.len();

        for i in 0..len {
            if (num.get(0..(i + 1)).unwrap().len() > 1 && num.get(0..1).unwrap() == "0") {
                return false;
            }
            // dbg!(num.get(0..(i + 1)).unwrap());
            let a = num.get(0..(i + 1)).unwrap().parse::<u128>().unwrap();
            for j in (i + 1)..len {
                if (num.get((i + 1)..(j + 1)).unwrap().len() > 1
                    && num.get((i + 1)..(i + 2)).unwrap() == "0")
                {
                    continue;
                }
                let b = num.get((i + 1)..(j + 1)).unwrap().parse::<u128>().unwrap();
                // dbg!("------");
                // dbg!(a, b);
                if (run(a, b, &num, j + 1)) {
                    return true;
                }
            }
        }
        return false;
    }
}

#[test]
fn test() {

    assert!(!Solution::is_additive_number("11235813213455890144".to_string()));

    assert!(Solution::is_additive_number("198019823962".to_string()));
    assert!(!Solution::is_additive_number("0235813".to_string()));

    assert!(Solution::is_additive_number("101".to_string()));

    assert!(Solution::is_additive_number("1235813".to_string()));
    assert!(Solution::is_additive_number(
        "11000000000110000000002".to_string()
    ));
    assert!(!Solution::is_additive_number("10".to_string()));

    assert!(!Solution::is_additive_number("1023".to_string()));

    assert!(run(1, 2, "35", 0));
}
