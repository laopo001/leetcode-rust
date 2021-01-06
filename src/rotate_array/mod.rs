struct Solution;


impl Solution {
    // 100%
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len() as i32;
        let t = k % len;
        let offset = len - (if t == 0 { len } else { t });
        // println!("{}",offset);
        if offset < 0 {
            return nums.reverse();
        }
        let (a, b) = nums.split_at_mut(offset as usize);
        let a_clone = a.to_owned();
        let b_clone = b.to_owned();
        drop(a);
        drop(b);
        nums.clear();
        nums.extend_from_slice(&b_clone);
        nums.extend_from_slice(&a_clone);
    }
}