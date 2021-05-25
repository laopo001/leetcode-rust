struct Solution;

fn run(mut a: i32, mut b: i32, num: &str, mut start: usize, arr: &mut Vec<i32>) -> bool {
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
            arr.push(c as i32);
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
    pub fn split_into_fibonacci(num: String) -> Vec<i32> {
        let len = num.len();
        let mut arr: Vec<i32> = vec![];
        for i in 0..len {
            if (num.get(0..(i + 1)).unwrap().len() > 1 && num.get(0..1).unwrap() == "0") {
                continue;
            }
            // dbg!(num.get(0..(i + 1)).unwrap());
            if let Ok(a) = num.get(0..(i + 1)).unwrap().parse::<i32>() {
                for j in (i + 1)..len {
                    if (num.get((i + 1)..(j + 1)).unwrap().len() > 1
                        && num.get((i + 1)..(i + 2)).unwrap() == "0")
                    {
                        continue;
                    }
                    if let Ok(b) = num.get((i + 1)..(j + 1)).unwrap().parse::<i32>() {
                        // dbg!("------");
                        // dbg!(a, b);
                        arr.clear();
                        arr.push(a as i32);
                        arr.push(b as i32);
                        if (run(a, b, &num, j + 1, &mut arr)) {
                            return arr;
                        }
                    } else {
                        continue;
                    }
                }
            } else {
                arr.clear();
                return arr;
            }
        }
        arr.clear();
        return arr;
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::split_into_fibonacci("31326395158253411".to_string()),
        vec![31, 32, 63, 95, 158, 253, 411]
    );
    assert_eq!(
        Solution::split_into_fibonacci(
            "74912134825162255812723932620170946950766784234934".to_string()
        ),
        vec![]
    );
    assert_eq!(Solution::split_into_fibonacci("0123".to_string()), vec![]);
    assert_eq!(
        Solution::split_into_fibonacci("123456579".to_string()),
        vec![123, 456, 579]
    );
    assert_eq!(
        Solution::split_into_fibonacci("11235813".to_string()),
        vec![1, 1, 2, 3, 5, 8, 13]
    );
}
