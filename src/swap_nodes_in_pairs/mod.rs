use crate::base::{ListNode, Solution};

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        return head.and_then(|mut a| match a.next {
            Some(mut b) => {
                a.next = Solution::swap_pairs(b.next);
                b.next = Some(a);
                Some(b)
            }
            None => Some(a),
        });
    }
}
