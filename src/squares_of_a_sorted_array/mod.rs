use crate::base::Solution;

impl Solution {
	pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
		let mut res = a.into_iter().map(|x| x * x).collect::<Vec<i32>>();
		res.sort();
		res
	}
	pub fn sorted_squares2(a: Vec<i32>) -> Vec<i32> {
		let temp = {
			let mut t = 0;
			let mut res = 0;
			for i in 0..a.len() {
				if a[i] < 0 {
					t = -a[i];
					res = i;
				}
				if a[i] == 0 {
					res = i;
					break;
				}
				if a[i] > 0 {
					if a[i] < t {
						res = i;
					}
					break;
				}
			}
			res
		};
		let mut res: Vec<i32> = vec![];
		res.push(a[temp] * a[temp]);
		let mut left = temp - 1;
		let mut right = temp + 1;
		while (left as i32) > -1 || right  < a.len() {
			if (left as i32)  <= -1 && right < a.len() {
				res.push(a[right] * a[right]);
				right += 1;
			}
			if (left as i32)  > -1 && right >= a.len() {
				res.push(a[left] * a[left]);
				left -= 1;
			}
			if (left as i32)  > -1 && right < a.len() {
				if 0 - a[left] > a[right] {
					res.push(a[right] * a[right]);
					right += 1;

				} else {
					res.push(a[left] * a[left]);
					left -= 1;
				}
			}
		}
		res
	}
}