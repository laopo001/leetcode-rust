struct Solution;

impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        if (nums.len() == 0 || nums.len() == 1) {
            return;
        }
        let mut b = false;
        for i in 1..nums.len() {
            b = !b;
            if nums[i] == nums[i - 1] {
                let mut a = true;
                for j in (i + 1)..nums.len() {
                    if (nums[j] != nums[i]) {
                        let t = nums[i];
                        nums[i] = nums[j];
                        nums[j] = t;
                        a = false;
                        break;
                    }
                }
                if (a) {
                    for j in 0..(i - 1) {
                        if (nums[j] > nums[i - 1]) == b
                            && j % 2 == 1
                            && nums[j - 1] < nums[i]
                            && nums[j + 1] < nums[i]
                        {
                            let t = nums[i];
                            nums[i] = nums[j];
                            nums[j] = t;
                            break;
                        }
                        if (nums[j] > nums[i - 1]) == b
                            && j % 2 == 0
                            && (j == 0 || nums[j - 1] > nums[i])
                            && nums[j + 1] > nums[i]
                        {
                            let t = nums[i];
                            nums[i] = nums[j];
                            nums[j] = t;
                            break;
                        }
                    }
                }
            }
            if (nums[i] > nums[i - 1]) != b {
                let t = nums[i];
                nums[i] = nums[i - 1];
                nums[i - 1] = t;
            }
        }
    }
}

#[test]
fn test() {
    let mut arr = vec![4, 5, 5, 6];
    Solution::wiggle_sort(&mut arr);
    dbg!(arr);
}
