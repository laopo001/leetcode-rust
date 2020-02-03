struct Solution;
// Runtime: 0 ms, faster than 100.00%
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in nums {
            res ^= i;
        }
        return res;
    }
}

#[test]
fn test() {
    assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
    assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
}
