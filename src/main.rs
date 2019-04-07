#![allow(unused)]
use leetcode::base::{num_to_string, string_to_num, Solution};
use leetcode::best_time_to_buy_and_sell_stock_with_cooldown;
use std::mem;

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
}
