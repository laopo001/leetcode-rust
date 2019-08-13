use crate::base::Solution;

impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        if n % 2 == 0 {
            return true;
        } else {
            return false;
        }
    }
    pub fn divisor_game2(n: i32) -> bool {
        let mut arr: Vec<i32> = vec![-1; n as usize + 1];
        arr[1] = 0;
        for i in 2..=n {
            for j in 1..i {
                // 0ms
                if arr[i as usize - j as usize] == 0 && i % j == 0
                // 4ms 判断整除计算消耗大于在数组中对比一个项
                // if i % j == 0 && arr[i as usize - j as usize] == 0
                {
                    // 4ms
                    arr[i as usize] = 1;
                    break;
                }
                arr[i as usize] = 0;
            }
        }
        // println!("{:?}",arr);
        return arr[n as usize] == 1;
    }
}
