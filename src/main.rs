#![allow(unused)]
use leetcode::base::{num_to_string, string_to_num, JsArray, ListNode, Solution, TreeNode};
use leetcode::longest_palindromic_substring;
use leetcode::remove_nth_node_from_end_of_list;
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
    let q = arr.find_mut(|&x| x == 1).unwrap();
    *q = *q + 9;
    println!("{:?}", *q);
    ///
    let mut a = vec![1, 2];
    let mut count = 0;
    a.for_each(|x| count += *x);
    println!("{:?},{:?}", a.map(|x| [*x]), count);
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
    //
    {
        let l = ListNode {
            val: 0,
            next: Some(Box::new(ListNode { val: 1, next: None })),
        };
        // println!("{:?}", (*(l.next.unwrap())));
        // let z = l.clone();
        // let x = l.clone();
        // println!("{:p}", &(*(x.next.unwrap())));
        // println!("{:p}", &(*(z.next.unwrap())));
        // dbg!(l == l.clone());

        let mut arr: Vec<*const ListNode> = vec![];
        let mut q = Some(Box::new(l));
        let res = Solution::remove_nth_from_end2(q, 1);
        println!("{:?}", res);

        let a: *mut i32 = &mut 1;
        let b: *mut i32 = &mut 2;
        let mut line: Vec<*mut i32> = vec![];
        line.push(a);
        line.push(b);

        unsafe {
            let a1 = line[0];
            println!("{:?}", *a1);
            let b1 = line[1];
            // *b1 = 3;
            let p = *b1;
            *b1 = 3;
            println!("{:?}", p);
        }
        // while let Some(node) = q {
        //     arr.push(&*node);
        //     q = node.next;
        // }
        // unsafe {
        //     println!("{:?}", *arr[0]);
        // }
    }
}
