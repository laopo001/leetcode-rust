use crate::base::{TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

fn run(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if s.is_none() && t.is_none() {
        return true;
    } else if s.is_some() && t.is_some() {
        let s_s = s.unwrap();
        if s_s.borrow().val == t.clone().unwrap().borrow().val {
            return Solution::is_subtree(
                s_s.borrow().left.clone(),
                t.clone().unwrap().borrow().left.clone(),
            ) && Solution::is_subtree(
                s_s.borrow().right.clone(),
                t.clone().unwrap().borrow().right.clone(),
            );
        } else {
            return false;
        }
    } else {
        return false;
    }
}

impl Solution {
    // 4ms 100%
    pub fn is_subtree(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if s.is_none() && t.is_none() {
            return true;
        } else if s.is_some() && t.is_some() {
            let s_s = s.unwrap();
            if s_s.borrow().val == t.clone().unwrap().borrow().val {
                return (run(
                    s_s.borrow().left.clone(),
                    t.clone().unwrap().borrow().left.clone(),
                ) && run(
                    s_s.borrow().right.clone(),
                    t.clone().unwrap().borrow().right.clone(),
                )) || Solution::is_subtree(s_s.borrow().left.clone(), t.clone())
                    || Solution::is_subtree(s_s.borrow().right.clone(), t);
            } else {
                return Solution::is_subtree(s_s.borrow().left.clone(), t.clone())
                    || Solution::is_subtree(s_s.borrow().right.clone(), t);
            }
        } else {
            return false;
        }
    }
}
