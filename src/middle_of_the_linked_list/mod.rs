use crate::base::ListNode;
struct Solution;
// Runtime: 0 ms, faster than 100.00%
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut res = vec![];
        let mut temp = head;
        while temp.is_some() {
            res.push(temp.clone());
            temp = temp.unwrap().next;
        }
        let index = res.len() / 2;
        res[index].clone()
    }
}
