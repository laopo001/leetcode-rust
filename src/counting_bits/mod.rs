use crate::base::Solution;

impl Solution {
	pub fn count_bits(num: i32) -> Vec<i32> {
		let mut i = 1;
		let mut res = vec![0];
		while i <= num {
			for j in 0..(i as usize) {
				res.push(res[j] + 1);
			}
			i = i * 2;
		}
		unsafe { res.get_unchecked(0..(num + 1) as usize).to_vec() }
	}
}