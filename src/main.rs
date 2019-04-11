#![allow(unused)]
use leetcode::base::{num_to_string, string_to_num, JsArray, Solution, TreeNode};
use leetcode::longest_palindromic_substring;
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
struct Point {
    data: [i32; 2],
    y: i32,
}

fn main() {
    #[derive(Debug)]
    struct Foo;
    trait Bar {
        fn baz(&self);
    }
    impl Bar for Foo {
        fn baz(&self) {
            println!("{:?}", self)
        }
    }
    fn static_dispatch<T>(t: &T)
    where
        T: Bar,
    {
        t.baz();
    }
    fn dynamic_dispatch(t: &Bar) {
        t.baz();
    }
    let foo = Foo;
    static_dispatch(&foo);
    dynamic_dispatch(&foo);
    ///
    let mut arr = vec![1];
    let q = arr.find_mut(Box::new(|&x| x == 1)).unwrap();
    *q = *q + 9;
    println!("{:?}", *q);
    ///
    let box_p = Box::new([1, 231]);
    let mut p = *box_p;
    p[0] = 999;
    println!("{:?}", p);
    println!("{:?}", *box_p);
    let mut a = 1;
    let a_ref = &mut a;
    let mut b = 2;
    let b_ref = &mut b;
    // a_ref = b_ref;
    mem::swap(a_ref, b_ref);
    println!("{:?}", a_ref);
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
