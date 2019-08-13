use crate::base::Solution;

impl Solution {
    // 0ms 100%
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut arr = stones;
        arr.sort();
        let len = arr.len();
        if len == 0 {
            return 0;
        }
        if len == 1 {
            return arr[0];
        }
        while arr.len() > 1 {
            println!("{}", 1);
            let a = arr.pop().unwrap();
            let b = arr.pop().unwrap();
            let c = a - b;
            if c != 0 {
                let mut b = true;
                for i in 0..arr.len() {
                    if arr[i] >= c {
                        b = false;
                        arr.insert(i, c);
                        break;
                    }
                }
                if b {
                    arr.push(c);
                }
            }
        }
        // if arr.len() == 0 {
        //     return 0;
        // }
        if arr.len() == 1 {
            return arr[0];
        }
        0
    }
}
