use crate::base::{ListNode, Solution};

impl Solution {
    pub fn remove_nth_from_end2(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut root = head;
        let mut arr: Vec<*mut ListNode> = vec![];
        while let Some(mut node) = root {
            arr.push(node.as_mut());
            root = node.next;
        }
        let len = arr.len();
        // println!("{:?}", len);
        if len == 0 {
            return None;
        }
        unsafe {
            // println!("{:?}", (*arr[0]).val);
            if len == n as usize {
                return Some(Box::new((*arr[1]).clone()));
            }
            let a = arr[len - n as usize - 2];
            // println!("{:?}", len - n as usize - 2);
            if n == 0 {
                println!("123");
                (*a).next = None;
            } else {
                let b = (*arr[len - n as usize]).clone();
                (*a).next = Some(Box::new(b));
            }
            println!("{:?}", (*arr[0]).val);
            return Some(Box::new((*arr[0]).clone()));
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
