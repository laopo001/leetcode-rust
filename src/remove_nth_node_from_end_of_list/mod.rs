use crate::base::{ListNode, Solution};

impl Solution {
    pub fn remove_nth_from_end2(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut root = head;
        let mut arr: Vec<*mut ListNode> = vec![];
        while let Some(mut node) = root {
            println!("{:?}", node.val);
            arr.push(node.as_mut());
            root = node.next;
        }
        // unsafe {
        //     for i in arr {
        //         println!("{:?}", *i);
        //     }
        // }
        let len = arr.len();
        if len == 0 {
            return None;
        }
        unsafe {
            if len == n as usize {
                return Some(Box::new((*arr[1]).clone()));
            }
            let a = arr[len - n as usize - 1];
            println!("==={:?}", (*a).val);
            if n == 0 {
                // println!("123");
                println!("----{:?}", (*a).val);
                (*a).next = None;
            } else {
                let b = arr[len - n as usize];
                // println!("----{:?},{:?}", (*a).val, (*b).val);
                // (*a).next = Some(Box::new(*b));
            }
            // println!("{:?}", (*arr[0]).val);
            return Some(Box::new((*arr[0]).clone()));
        }
        // return None;
    }
    pub fn remove_nth_from_end3(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut root = head;
        let mut arr: Vec<i32> = vec![];
        while let Some(node) = root {
            arr.push(node.val);
            root = node.next;
        }
        arr.remove(arr.len() - n as usize);
        // println!("{:?}", arr);
        if arr.len() == 0 {
            return None;
        } else {
            // let mut res = ListNode {
            //     val: arr.remove(0),
            //     next: None,
            // };
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
