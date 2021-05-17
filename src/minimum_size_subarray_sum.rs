struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, mut nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut z = 0;
        for i in nums.iter() {
            sum += *i;
            z += 1;
            if(sum >= target){
                break;
            }
        }
        z
    }
}
