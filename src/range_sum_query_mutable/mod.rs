struct NumArray {
    nums: Vec<i32>,
}
// Runtime: 28 ms, faster than 20.00%
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        NumArray { nums }
    }

    fn update(&mut self, i: i32, val: i32) {
        self.nums[i as usize] = val;
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        let mut res = 0;
        for i in i..=j {
            res += self.nums[i as usize];
        }
        res
    }
}
