use crate::base::{ListNode};

struct Solution;

impl Solution {
    pub fn get_decimal_value(mut head: Option<Box<ListNode>>) -> i32 {
        let mut nums = vec![];
        while let Some(b) = head {
            nums.push(b.val);
            head = b.next;
        }
        nums.reverse();
        let mut res = 0;
        for i in 0..nums.len() {
            res += nums[i] * 2_i32.pow(i as u32);
        }
        res
    }
}
