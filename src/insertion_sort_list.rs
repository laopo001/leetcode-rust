use crate::base::ListNode;
struct Solution;

use std::ptr;
trait AsMutPtr<T> {
    fn as_mut_ptr(&mut self) -> *mut T;
}

impl<'a, T> AsMutPtr<T> for Option<T> {
    fn as_mut_ptr(&mut self) -> *mut T {
        match self {
            Some(value) => value as *mut T,
            None => ptr::null_mut(),
        }
    }
}

impl Solution {
    pub fn insertion_sort_list2(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        unsafe {
            let mut res = Some(Box::new(ListNode::new(std::i32::MIN)));
            while let Some(node) = head {
                let mut is_add = false;
                let mut pre_node_ptr: *mut Box<ListNode> = std::ptr::null_mut();
                let mut temp = res.as_mut_ptr();
                while (!temp.is_null()) {
                    let node2 = &mut *temp;
                    pre_node_ptr = temp;
                    if (node2.val <= node.val
                        && node2.next.is_some()
                        && node2.next.as_ref().unwrap().val > node.val)
                    {
                        is_add = true;
                        (*pre_node_ptr).next = Some(Box::new(ListNode {
                            val: node.val,
                            next: node2.next.clone(),
                        }));
                        break;
                    }
                    temp = node2.next.as_mut_ptr();
                }
                if (!is_add) {
                    is_add = false;
                    (*pre_node_ptr).next = Some(Box::new(ListNode {
                        val: node.val,
                        next: None,
                    }));
                }
                head = node.next;
            }
            return res.unwrap().next;
        }
    }
    pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut dummy = Box::new(ListNode::new(-1));
        let mut prev = &mut dummy;

        while let Some(mut node) = head {
            let temp = node.next.take();

            /* Before insert, the prev is at the last node of the sorted list.
            Only the last node's value is larger than the current inserting node
            should we move the temp back to the head*/
            if prev.val > node.val {
                prev = &mut dummy;
            }
            //find the right place to insert
            while prev.next.is_some() && prev.next.as_ref().unwrap().val < node.val {
                prev = prev.next.as_mut().unwrap();
            }
            //insert between pre and pre.next
            node.next = prev.next.take();
            prev.next = Some(node);
            // prev = dummy; // Don't set prev to the head of the list after insert
            head = temp;
        }

        dummy.next
    }
}

#[test]
fn test() {
    let a = Solution::insertion_sort_list2(Some(Box::new(ListNode {
        val: 4,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        })),
    })));
    dbg!(a);
    assert!(true);
}
