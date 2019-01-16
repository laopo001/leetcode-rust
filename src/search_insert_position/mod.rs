struct Solution;
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        for i in 0..nums.len() {
            let item = nums[i];
            if (target == item) {
                return i as i32;
            }
            if (target < item) {
                return (i - 1) as i32;
            }
        }
        return nums.len() as i32;
    }
}

#[test]
fn test() {
    Solution::search_insert(vec![1, 3, 5, 6], 2);
}
