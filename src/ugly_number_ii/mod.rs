use crate::base::Solution;

impl Solution {
	pub fn nth_ugly_number(n: i32) -> i32 {
		let n = n as usize;
		let mut res = vec![1];
		res[0] = 1;
		let mut i2 = 0;
		let mut i3 = 0;
		let mut i5 = 0;
		while res.len() < n {
			let num2 = res[i2] * 2;
			let num3 = res[i3] * 3;
			let num5 = res[i5] * 5;
			let item = std::cmp::min(num2, std::cmp::min(num3, num5));
//			println!("{}",item);
			if num2 == item {
				i2 += 1;
			}
			if num3 == item {
				i3 += 1;
			}
			if num5 == item {
				i5 += 1;
			}
			res.push(item);
		}
		res[n - 1]
	}
}