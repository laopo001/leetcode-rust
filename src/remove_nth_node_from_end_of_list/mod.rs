use crate::base::{ListNode};
struct Solution;
impl Solution {
    // error
    pub fn remove_nth_from_end2(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        unsafe {
            let mut head = head;
            let mut front: *mut Option<Box<ListNode>> = &mut head;
            let mut tail: *mut Option<Box<ListNode>> = &mut head;
            for _ in 0..n {
                front = &mut (*front).as_mut().unwrap().next;
            }
            if (*front).is_none() {
                return head.take().unwrap().next;
            }
            loop {
                front = &mut (*front).as_mut().unwrap().next;
                if (*front).is_none() {
                    break;
                }
                tail = &mut (*tail).as_mut().unwrap().next;
            }
            (*tail).as_mut().unwrap().next =
                (*tail).as_mut().unwrap().next.as_mut().unwrap().next.take();
            head
        }
    }
    pub fn remove_nth_from_end3(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut root = head;
        let mut arr: Vec<i32> = vec![];
        while let Some(node) = root {
            arr.push(node.val);
            root = node.next;
        }
        arr.remove(arr.len() - n as usize);
        if arr.len() == 0 {
            return None;
        } else {
            let mut z = None;
            arr.reverse();
            for i in arr {
                z = Some(Box::new(ListNode { val: i, next: z }));
            }
            return z;
        }
    }
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut len = 0;
        {
            let mut p = dummy_head.as_ref();
            while p.unwrap().next.is_some() {
                len += 1;
                p = p.unwrap().next.as_ref();
            }
        }
        let idx = len - n;
        {
            let mut p = dummy_head.as_mut();
            for _ in 0..(idx) {
                p = p.unwrap().next.as_mut();
            }
            let next = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
            p.as_mut().unwrap().next = next;
        }
        dummy_head.unwrap().next
    }
}
