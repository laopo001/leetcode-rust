use std::collections::BinaryHeap;

struct KthLargest {
    arr:Vec<i32>,
}

fn addBinaryHeap(arr:&mut Vec<i32>,val:i32) {
    if *arr.first().unwrap() < val {
        arr.push(val);
        arr.sort();
        arr.remove(0);
    }
}

impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut arr= vec![0;k as usize];

        for  item  in nums.into_iter(){
            addBinaryHeap(&mut arr, item);
        }
        KthLargest {
            arr,
        }
    }
    
    fn add(&mut self, val: i32) -> i32 {
        addBinaryHeap(&mut self.arr, val);
        *self.arr.first().unwrap()
    }
}
