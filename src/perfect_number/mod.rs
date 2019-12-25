struct Solution;
// Runtime: 0 ms, faster than 100.00%
impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num <= 1 {
            return false;
        }
        let mut t = 2;
        let mut arr: Vec<i32> = vec![1];
        while t * t < num {
            let res = num % t;
            if res == 0 {
                arr.push(t);
                arr.push(num / t);
            }
            t += 1;
        }
        // println!("{:?}", arr);
        arr.into_iter().fold(0, |a, b| a + b) == num
    }
}

#[test]
fn test_check_perfect_number() {
    assert_eq!(Solution::check_perfect_number(28), true);
}
