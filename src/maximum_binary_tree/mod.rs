use crate::base::{TreeNode};
struct Solution;
use std::cell::RefCell;
use std::rc::Rc;

fn find_max_index(nums: &Vec<i32>) -> Option<(i32, usize)> {
    if nums.len() == 0 {
        return None;
    }
    let mut index = (i32::min_value(), 0);
    for i in 0..nums.len() {
        if nums[i] > index.0 {
            index.0 = nums[i];
            index.1 = i;
        }
    }
    return Some(index);
}

impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let index = find_max_index(&nums);
        if index == None {
            return None;
        } else {
            let (val, i) = index.unwrap();
            let mut node = TreeNode::new(val);
            let left: Vec<i32> = nums.split_at(i).0.into();
            node.left = Solution::construct_maximum_binary_tree(left);
            let right: Vec<i32> = nums.split_at(i + 1).1.into();
            node.right = Solution::construct_maximum_binary_tree(right);
            return Some(Rc::new(RefCell::new(node)));
        }
    }
}
