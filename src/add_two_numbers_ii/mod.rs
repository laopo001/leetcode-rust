use crate::base::ListNode;
struct Solution;

// 16 ms, faster than 9.09% 
impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut nums = vec![];
        let mut nums2 = vec![];
        while let Some(n) = l1 {
            nums.push(n.val);
            l1 = n.next;
        }
        nums.reverse();
        while let Some(n) = l2 {
            nums2.push(n.val);
            l2 = n.next;
        }
        nums2.reverse();
        let len = if nums.len() > nums2.len() {
            nums.len()
        } else {
            nums2.len()
        };
        let mut z = 0;
        let mut arr = vec![];
        for i in 0..len {
            let mut c = nums.get(i).unwrap_or(&0) + nums2.get(i).unwrap_or(&0) + z;
            if c > 9 {
                z = 1;
                c -= 10;
            } else {
                z = 0;
            }
            arr.push(c)
        }
        if z == 1 {
            arr.push(1)
        }
        let mut next: Option<Box<ListNode>> = None;
        for i in arr {
            let mut t = ListNode::new(i);
            t.next = next.clone();
            next = Some(Box::new(t));
        }
        return next;
    }
}
