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

#[derive(PartialEq, Eq, Clone, Debug)]
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
    fn find_index<F>(&self, cb: F) -> Option<usize>
    where
        F: Fn(&T) -> bool;
    fn find<F>(&self, cb: F) -> Option<&T>
    where
        F: Fn(&T) -> bool;
    fn find_mut<F>(&mut self, cb: F) -> Option<&mut T>
    where
        F: Fn(&T) -> bool;
    fn for_each<F>(&mut self, cb: F)
    where
        F: FnMut(&mut T);
    fn filter<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool;
    fn map<F, K>(&mut self, f: F) -> Vec<K>
    where
        F: FnMut(&T) -> K;
    fn reduce<F, K>(&self, f: F, c: K) -> K
    where
        F: FnMut(&K, &T) -> K;
    // fn slice<R>(&self, r: R) -> &[T]
    // where
    //     R: std::ops::RangeBounds<usize>;
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
    fn find_index<F>(&self, cb: F) -> Option<usize>
    where
        F: Fn(&T) -> bool,
    {
        for i in 0..self.len() {
            if cb(&self[i]) {
                return Some(i);
            }
        }
        return None;
    }
    fn find<F>(&self, cb: F) -> Option<&T>
    where
        F: Fn(&T) -> bool,
    {
        for i in self.iter() {
            if cb(i) {
                return Some(i);
            }
        }
        return None;
    }
    fn find_mut<F>(&mut self, cb: F) -> Option<&mut T>
    where
        F: Fn(&T) -> bool,
    {
        for i in self.iter_mut() {
            if cb(i) {
                return Some(i);
            }
        }
        return None;
    }
    fn for_each<F>(&mut self, mut cb: F)
    where
        F: FnMut(&mut T),
    {
        for i in self.iter_mut() {
            cb(i);
        }
    }
    fn filter<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.retain(f)
    }
    fn map<F, K>(&mut self, mut f: F) -> Vec<K>
    where
        F: FnMut(&T) -> K,
    {
        let mut arr: Vec<K> = vec![];
        for i in self.iter_mut() {
            arr.push(f(i));
        }
        return arr;
    }
    fn reduce<F, K>(&self, mut f: F, mut c: K) -> K
    where
        F: FnMut(&K, &T) -> K,
    {
        for i in self.iter() {
            c = f(&c, i);
        }
        return c;
    }
}

#[test]
fn test_find_mut() {
    let mut arr = vec![1];
    let q = arr.find_mut(|&x| x == 1).unwrap();
    *q = *q + 9;
    assert_eq!(10, *q);
}

#[test]
fn test_for_each() {
    let mut arr = vec![1];
    let mut count = 0;
    arr.for_each(|x| count += *x);
    assert_eq!(1, count);
}

#[test]
fn test_map() {
    let mut arr = vec![1];
    assert_eq!(vec![[1]], arr.map(|x| [*x]));
}
#[derive(Debug, PartialEq)]
struct A(i32);

#[test]
fn test_reduce() {
    let mut arr = vec![1, 2];
    assert_eq!(A(3), arr.reduce(|x, y| A(x.0 + y), A(0)));
}

pub trait Context {
    type Output;
    fn read(&self) -> Self::Output;
}

struct Book {}
impl Context for Book {
    type Output = i32;
    fn read(&self) -> Self::Output {
        println!("read");
        0
    }
}

struct Phone {}
impl Context for Phone {
    type Output = u32;
    fn read(&self) -> Self::Output {
        println!("read");
        0
    }
}

struct Data<T:Context> {
    out:T::Output, // 这个类型怎么填
}
impl<T:Context> Data<T> {
    fn new(a:T) -> Self {
        let out =  a.read();
        Data {out}
    }
}

fn test(){
    let a = Data::new(Phone{});
    let b = a.out;
}