use crate::base::Solution;

impl Solution {
	pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
		let mut t = 1;
		let mut res: Vec<i32> = vec![];
		let mut b = false;
		let mut left = 1;
		let mut right = n + 1;
		while left < right {
			if k > t {
				if b {
					res.push(right-1 );
					right -= 1;
					b = false;
				} else {
					res.push(left);
					left += 1;
					b = true;
				}
				t += 1;
			} else {
				if b {
					res.push(right -1);
					right -= 1;
				} else {
					res.push(left);
					left += 1;
				}
			}
		}
		res
	}
}