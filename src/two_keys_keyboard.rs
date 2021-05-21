struct Solution;

impl Solution {
    // 96 ms, faster than 33.33%
    pub fn min_steps(n: i32) -> i32 {
        if (n == 1) {
            return 0;
        }

        let mut arr = vec![0 as i32; (n + 1) as usize];
        arr[1] = 0;

        let mut i: usize = 2;
        loop {
            if (i > n as usize) {
                break;
            }
            for x in i..=(n as usize) {
                if (x == i) {
                    if (arr[x] == 0) {
                        arr[x] = x as i32;
                    }
                } else if (x % i == 0) {
                    arr[x] = arr[i] + (x / i) as i32;
                }
            }
            i += 1;
            // dbg!(arr.clone());
        }

        arr[n as usize]
    }
}
#[test]
fn test() {
    assert_eq!(Solution::min_steps(6), 5);
    assert_eq!(Solution::min_steps(11), 11);
    assert_eq!(Solution::min_steps(12), 7);
}
