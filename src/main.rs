#![allow(unused)]
use std::cell::RefCell;
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::i32;
use std::rc::Rc;

fn num_unique_emails(emails: &mut Vec<String>) -> i32 {
    let mut set: HashSet<String> = HashSet::new();
    for item in emails.into_iter() {
        let arr: Vec<_> = item.split("@").collect();
        let mut nemail = String::new();
        let left: &str = arr[0];
        let right: &str = arr[1];
        let mut t: String = left.replace(".", "");
        let arr2: Vec<_> = t.match_indices("+").collect();
        if arr2.len() > 0 {
            t = t[0..arr2[0].0].to_string();
        }
        nemail = t + "@" + right;
        set.insert(nemail);
    }
    return set.len() as i32;
}

fn repeated_n_times(a: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, bool> = HashMap::new();
    for item in a {
        if (!map.contains_key(&item)) {
            map.insert(item, true);
        } else {
            return item;
        }
    }
    return 123;
}

fn to_lower_case(str: String) -> String {
    // for i in 0..str.len() {
    //     let c = str.get(i..(i+1));
    // }
    let arr = str.as_bytes();
    let mut res = Vec::new();
    for item in arr {
        if *item <= (65 as u8) && *item <= (90 as u8) {
            res.push(item + 32);
        } else {
            res.push(*item);
        }
    }
    return String::from_utf8(res).unwrap();
}

struct MyStack {
    r: RefCell<Vec<i32>>,
}

/** You can modify self type for your need. */
impl MyStack {
    /** Initialize your data structure here. */
    fn new() -> Self {
        MyStack {
            r: RefCell::new(Vec::new()),
        }
    }
    /** Push element x onto stack. */

    fn push(&self, x: i32) {
        self.r.borrow_mut().push(x);
    }
    /** Removes the element on top of the stack and returns that element. */

    fn pop(&self) -> i32 {
        return self.r.borrow_mut().pop().unwrap();
    }
    /** Get the top element. */

    fn top(&self) -> i32 {
        *self.r.borrow().first().unwrap()
    }
    /** Returns whether the stack is empty. */

    fn empty(&self) -> bool {
        self.r.borrow().is_empty()
    }
}

fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // let coumnLen = a.len();
    // let rowLen = a[0].len();
    let mut b: Vec<Vec<i32>> = Vec::new();
    for row in a {
        let mut new_row: Vec<i32> = Vec::new();
        // row.reserve(row.len());
        for i in 0..row.len() {
            let curr_index = row.len() - 1 - i;
            let curr = if row[curr_index] == 0 { 1 } else { 0 };
            new_row.push(curr);
        }
        b.push(new_row);
    }
    return b;
}

fn index_of_vec<T: PartialEq>(v: &Vec<T>, i: T) -> Option<usize> {
    for item in 0..v.len() {
        if i == v[item] {
            return Some(item);
        }
    }
    return None;
}

fn find_words(words: Vec<String>) -> Vec<String> {
    let arr = vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'];
    let arr1 = vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'];
    let arr2 = vec!['z', 'x', 'c', 'v', 'b', 'n', 'm'];
    let mut res: Vec<String> = Vec::new();
    for item in words {
        let mut b = true;
        let lowers = item.to_ascii_lowercase();
        for c in lowers.chars() {
            if index_of_vec(&arr, c) == None {
                b = false;
                break;
            }
        }
        if (b) {
            res.push(item);
            continue;
        }
        b = true;
        for c in lowers.chars() {
            if index_of_vec(&arr1, c) == None {
                b = false;
                break;
            }
        }
        if (b) {
            res.push(item);
            continue;
        }
        b = true;
        for c in lowers.chars() {
            if index_of_vec(&arr2, c) == None {
                b = false;
                break;
            }
        }
        if (b) {
            res.push(item);
            continue;
        }
    }
    return res;
}

fn to_decimal(n: i32, d: i32) -> String {
    let mut res = "".to_string();
    let mut n = n;
    while n != 0 {
        let t = n % d;
        res = t.to_string() + res.as_str();
        n = (n - t) / d;
    }
    return res.to_string();
}

fn binary_gap(n: i32) -> i32 {
    let w: String = to_decimal(n, 2);
    let mut len = 0;
    let mut i = 0;
    let mut cout = 0;
    let mut a = 0;
    let mut b = 0;
    for c in w.chars() {
        i = i + 1;
        if (c == '1') {
            if (cout == 0) {
                a = i;
                cout = cout + 1;
            }
            if (cout == 1) {
                b = i;
                len = cmp::max(b - a, len);
                a = b;
                // b = 0;
            }
        }
    }
    return len;
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(r) = &root {
        if r.borrow().val < val {
            return search_bst(r.borrow().left.clone(), val);
        }
        if r.borrow().val > val {
            return search_bst(r.borrow().right.clone(), val);
        }
    }
    return root;
}

fn main() {
    let mut emails = vec!["12@3".to_string()];
    let map: HashMap<String, bool> = HashMap::new();
    num_unique_emails(&mut emails);
    println!("Hello, world!");
}

#[derive(Debug, Copy, Clone)]
enum EmailParseState {
    Name,
    Plus,
    Domain,
}

#[derive(Debug)]
struct Solution {}
impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut unique_emails = HashSet::new();
        for email in emails {
            let nemail = Solution::parse_email(&email);
            unique_emails.insert(nemail);
        }
        unique_emails.len() as i32
    }

    fn parse_email(email: &String) -> String {
        let mut nemail = String::new();
        let mut state = EmailParseState::Name;

        for c in email.chars() {
            let (new_state, ch) = match (state, c) {
                (EmailParseState::Name, '.') => (state, None),
                (EmailParseState::Name, '+') => (EmailParseState::Plus, None),
                (EmailParseState::Name, '@') => (EmailParseState::Domain, Some(c)),
                (EmailParseState::Name, _) => (state, Some(c)),
                (EmailParseState::Plus, '@') => (EmailParseState::Domain, Some(c)),
                (EmailParseState::Plus, _) => (state, None),
                (EmailParseState::Domain, _) => (state, Some(c)),
            };
            if let Some(c) = ch {
                nemail.push(c);
            }
            state = new_state;
        }
        nemail
    }
}
