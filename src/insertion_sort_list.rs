use crate::base::ListNode;
struct Solution;

impl Solution {
    // pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    //     let mut res: Box<ListNode> = Box::new(ListNode::new(std::i32::MIN));
    //     let mut prev = &mut res.next;
    //     let mut temp = Some(Box::new(ListNode::new(std::i32::MIN)));
    //     while let Some(node) = head {
    //         if (prev.is_none()) {
    //             std::mem::swap(
    //                 prev,
    //                 &mut Some(Box::new(ListNode {
    //                     val: node.val,
    //                     next: None,
    //                 })),
    //             );
    //             head = node.next;
    //             continue;
    //         }

    //         unsafe {
    //             let mut pre_node_ptr: *mut ListNode = std::ptr::null_mut();
    //             let mut is_add = false;
    //             temp = (*prev);
    //             while let Some(mut node2) = (temp) {
    //                 let mut next = node2.next.take();
    //                 if node2.val < node.val {
    //                     is_add = true;
    //                     (*pre_node_ptr).next = Some(Box::new(ListNode {
    //                         val: node.val,
    //                         next: next,
    //                     }));

    //                     break;
    //                 }
    //                 pre_node_ptr = node2.as_mut() as *mut ListNode;

    //                 temp = next;
    //             }
    //             if (!is_add) {
    //                 is_add = true;
    //                 (*pre_node_ptr).next = Some(Box::new(ListNode {
    //                     val: node.val,
    //                     next: None,
    //                 }));
    //             }
    //         }
    //         head = node.next;
    //     }
    //     return res.next;
    // }
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
    let a = Solution::insertion_sort_list(Some(Box::new(ListNode {
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
