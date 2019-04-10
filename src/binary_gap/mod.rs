use std::cmp;
#[allow(dead_code)]
pub fn to_decimal(n: i32, d: i32) -> String {
    let mut res = "".to_string();
    let mut n = n;
    while n != 0 {
        let t = n % d;
        res = t.to_string() + res.as_str();
        n = (n - t) / d;
    }
    return res.to_string();
}
#[allow(dead_code, unused_assignments)]
pub fn binary_gap(n: i32) -> i32 {
    let w: String = to_decimal(n, 2);
    let mut len = 0;
    let mut i = 0;
    let mut cout = 0;
    let mut a = 0;
    let mut b = 0;
    for c in w.chars() {
        i = i + 1;
        if c == '1' {
            if cout == 0 {
                a = i;
                cout = cout + 1;
            }
            if cout == 1 {
                b = i;
                len = cmp::max(b - a, len);
                a = b;
                // b = 0;
            }
        }
    }
    return len;
}
