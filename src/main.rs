#![allow(unused)]
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

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
