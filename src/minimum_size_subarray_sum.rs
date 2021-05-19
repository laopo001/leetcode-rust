use core::num;

struct Solution;


impl Solution {
    //  0 ms, faster than 100.00%
    pub fn min_sub_array_len(target: i32, mut nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut res = std::i32::MAX;
        let mut start = 0;
        let mut end = 0;
        let len = nums.len();
        loop {
            if (sum >= target) {
                res = std::cmp::min(res, (end - start) as i32);
                sum -= nums[start];
                start += 1;
            } else {
                if(end == len){
                    break;
                }
                sum += nums[end];
                end += 1;
            }
        }
        if (res == std::i32::MAX){
            0
        } else {
          res 
        }

    }
}
