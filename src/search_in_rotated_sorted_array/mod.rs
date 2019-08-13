use crate::base::Solution;

fn run(left: usize, right: usize, nums: &Vec<i32>, target: i32) -> i32 {
    let len = right - left;
    if len <= 0 {
        return -1;
    }
    let left_v = nums[left];
    let right_index = right - 1;
    let right_v = nums[right_index];
    let middle_index = left + (len as f32 / 2.0).floor() as usize;
    let middle = nums[middle_index];
    if middle == target {
        return middle_index as i32;
    }
    if left_v == target {
        return left as i32;
    }
    if right_v == target {
        return right_index as i32;
    }
    if right_v > left_v {
        if target < middle {
            return run(left, middle_index, &nums, target);
        } else {
            return run(middle_index + 1, right, &nums, target);
        }
    }

    if left_v < middle {
        if target > middle {
            return run(middle_index + 1, right, &nums, target);
        } else {
            if target < left_v {
                return run(middle_index + 1, right, &nums, target);
            } else {
                return run(left, middle_index, &nums, target);
            }
        }
    } else {
        if target < middle {
            return run(left, middle_index, &nums, target);
        } else {
            if target > right_v {
                return run(left, middle_index, &nums, target);
            } else {
                return run(middle_index + 1, right, &nums, target);
            }
        }
    }
}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        return run(0, nums.len(), &nums, target);
    }
}
