#![allow(unused)]
use crate::base::ListNode;
struct Solution;
// Runtime: 0 ms, faster than 100.00%
impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 0 || head == None {
            return head;
        }
        let mut vec: Vec<i32> = vec![];
        // let t = head.clone();
        while let Some(foo) = head {
            vec.push(foo.val);
            head = foo.next;
        }
        let len = vec.len();
        let t = (k as usize) % len;
        let mut vec2 = vec.split_off(len - t);
        vec2.extend(vec);
        let mut h: Option<Box<ListNode>> = None;
        // println!("{:?}", vec2);
        for x in vec2.into_iter().rev() {
            let mut node = ListNode::new(x);
            node.next = h;
            h = Some(Box::new(node));
        }
        h
    }
}
