#[allow(dead_code)]
struct KthLargest {
    arr: Vec<i32>,
}
#[allow(dead_code, non_snake_case)]
fn addBinaryHeap(arr: &mut Vec<i32>, val: i32) {
    if *arr.first().unwrap() < val {
        arr.push(val);
        arr.sort();
        arr.remove(0);
    }
}

impl KthLargest {
    #[allow(dead_code)]
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut arr = vec![i32::min_value(); k as usize];

        for item in nums.into_iter() {
            addBinaryHeap(&mut arr, item);
        }
        KthLargest { arr }
    }
    #[allow(dead_code)]
    fn add(&mut self, val: i32) -> i32 {
        addBinaryHeap(&mut self.arr, val);
        *self.arr.first().unwrap()
    }
}
