#![allow(unused)]

use std::cell::RefCell;
use std::rc::Rc;

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

#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}

pub struct Solution;

pub fn index_of_vec<T: PartialEq>(v: &Vec<T>, i: T) -> Option<usize> {
  for item in 0..v.len() {
    if i == v[item] {
      return Some(item);
    }
  }
  return None;
}

pub fn num_to_vec(n: i32, d: i32) -> Vec<i32> {
  let mut res: Vec<i32> = vec![];
  let mut n = n;
  while n != 0 {
    let t = n % d;
    res.push(t);
    n = (n - t) / d;
  }
  return res;
}

pub fn num_to_string(n: i32, d: i32) -> String {
  let mut res = "".to_string();
  let mut n = n;
  while n != 0 {
    let t = n % d;
    res = t.to_string() + res.as_str();
    n = (n - t) / d;
  }
  return res.to_string();
}

pub fn string_to_num(s: String, d: i32) -> i32 {
  let mut res = 0;
  let bytes = s.as_bytes();
  for i in 0..bytes.len() {
    let index = bytes.len() - 1 - i;
    res += (bytes[i] as i32 - 48) * d.pow(index as u32);
  }
  return res;
}

#[test]
fn test_string_to_num() {
  assert_eq!(string_to_num("12".to_string(), 10), 12);
}

pub trait JsArray<T> {
  // 方法从数组中删除第一个元素
  fn shift(&mut self) -> Option<T>;
  fn unshift(&mut self, elm: T);
  fn find(&self, cb: Box<Fn(&T) -> bool>) -> Option<&T>;
  fn find_mut(&mut self, cb: Box<Fn(&T) -> bool>) -> Option<&mut T>;
}

impl<T> JsArray<T> for Vec<T> {
  fn shift(&mut self) -> Option<T> {
    if self.len() == 0 {
      return None;
    } else {
      return Some(self.remove(0));
    }
  }
  fn unshift(&mut self, elm: T) {
    self.insert(0, elm);
  }
  fn find(&self, cb: Box<Fn(&T) -> bool>) -> Option<&T> {
    for i in self.iter() {
      if cb(i) {
        return Some(i);
      }
    }
    return None;
  }
  fn find_mut(&mut self, cb: Box<Fn(&T) -> bool>) -> Option<&mut T> {
    for i in self.iter_mut() {
      if cb(i) {
        return Some(i);
      }
    }
    return None;
  }
}
