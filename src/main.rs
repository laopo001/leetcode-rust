#![allow(unused)]
use leetcode::base::{num_to_string, string_to_num, Solution, TreeNode};
use leetcode::longest_palindromic_substring;
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

fn main() {
    let a = Rc::new(RefCell::new(TreeNode::new(5)));
    // {
    //     let t = a.borrow();
    //     let val = t.val + 5;
    //     drop(t);
    //     let mut z = a.borrow_mut();
    //     z.val = val;
    // }
    // let t = || a.borrow().val + 5;
    // a.borrow_mut().val = (|| a.borrow().val + 5)();
    a.borrow_mut().val += 5;
    a.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    println!("{:?}", a.borrow().val);
    ///
    ///
    let s = Solution::longest_palindrome2("aabaaa".to_string());
    println!("{:?}", s);
    println!("Hello, world!");

    ///
    let arr = [
        Rc::new(RefCell::new("Hello")),
        Rc::new(RefCell::new("World")),
    ];
    let s = arr
        .iter()
        .map(|s| {
            let c = Rc::clone(s);
            // format!("{}", c.borrow())
            // This line works fine
            return format!("{}", c.borrow());
        })
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", s);
}
