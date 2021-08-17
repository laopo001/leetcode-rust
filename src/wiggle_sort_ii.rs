struct Solution;

impl Solution {
    // 8 ms, faster than 100.00%  
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        nums.sort();
        // dbg!(nums.clone());
        let mut res: Vec<i32> = vec![];
        let mut a = nums.len() / 2 - (if (nums.len() % 2 == 0) { 1 } else { 0 });
        let t = a;
        let mut b = nums.len() - 1;
        loop {
            if (b == t) {
                res.push(nums[a]);
                break;
            }
            // dbg!(a, b);
            res.push(nums[a]);
            res.push(nums[b]);
            if a == 0 {
                break;
            }
            a -= 1;
            b -= 1;
        }
        for i in 0..nums.len() {
            nums[i] = res[i];
        }
    }
}

#[test]
fn test() {
    let mut arr = vec![1, 4, 3, 4, 1, 2, 1, 3, 1, 3, 2, 3, 3];
    Solution::wiggle_sort(&mut arr);
    dbg!(arr);
}
