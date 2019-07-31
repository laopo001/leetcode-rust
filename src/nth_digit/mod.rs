use crate::base::Solution;

impl Solution {
	// 0 ms, faster than 100.00%
	pub fn find_nth_digit(n: i32) -> i32 {
		// if n <= 9 {
		// 	return n;
		// }
		let mut n = n as u32;
		let mut i = 0;
		let mut j = 1;
		let mut q = 0;
		while n > 9 * 10_u32.pow(i) * j {
			q += 9 * 10_u32.pow(i);
			n -= 9 * 10_u32.pow(i) * j;
			i += 1;
			j += 1;
		}
		// println!("{},{},{}", q, j, n);
		let z = ((n as f32) / (j as f32)).ceil() as u32;
		let p = q + z;
		// let nth = ((n-(z-1)*j) - 1) as usize;
		let nth = ((n-1) % j) as usize;
		let s = p.to_string();
		// println!("{},{},{}", p, nth, s);
		return s.as_bytes()[nth] as i32 - 48;
	}
}