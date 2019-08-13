use crate::base::Solution;

impl Solution {
    pub fn sort_array_by_parity_ii(a: Vec<i32>) -> Vec<i32> {
        let mut arr = a;
        let mut odd: Vec<i32> = vec![];
        let mut even: Vec<i32> = vec![];

        for i in arr.iter() {
            if *i % 2 == 0 {
                even.push(*i);
            } else {
                odd.push(*i);
            }
        }

        for i in 0..(arr.len()) {
            if i % 2 == 0 {
                arr[i] = even.pop().unwrap();
            } else {
                arr[i] = odd.pop().unwrap();
            }
        }
        return arr;
    }
}
