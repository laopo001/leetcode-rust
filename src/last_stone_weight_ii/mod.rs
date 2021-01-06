struct Solution;

impl Solution {
    // 0ms 100%
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let len = stones.len();
        let mut sum = 0;
        for i in 0..len {
            sum += stones[i];
        }
        let mut f = vec![false; (sum as usize) + 1];
        f[0] = true;
        for i in 0..len {
            let z = stones[i];
            for j in (z..=sum / 2).rev() {
                f[j as usize] = f[j as usize] || f[j as usize - stones[i] as usize];
            }
        }
        for i in (0..=sum / 2).rev() {
            if f[i as usize] {
                return sum - i - i;
            }
        }
        return sum;
    }
}
