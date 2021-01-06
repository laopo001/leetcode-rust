struct Solution;

impl Solution {
	// 12 ms, faster than 100.00%
	pub fn add_to_array_form(a: Vec<i32>, k: i32) -> Vec<i32> {
		let mut b: Vec<i32> = vec![];
		let mut t = k;
		while t / 10 != 0 {
			b.push(t % 10);
			t /= 10;
		}
		b.push(t);
		let mut a = a;
		a.reverse();
		let len = if a.len() > b.len() { a.len() } else {
			std::mem::swap(&mut a,&mut b);
			a.len()
		};
		let mut t = 0;
		for i in 0..len {
			let p = *a.get(i).unwrap_or(&0);
			let q = *b.get(i).unwrap_or(&0);
			let mut res = p + q + t;
			// println!("{:?}", res);
			if res >= 10 {
				res = res - 10;
				t = 1;
			} else {
				t = 0;
			}
			a[i] = res;
		}
		if t == 1 {
			a.push(t);
		}
		a.reverse();
		a
	}
}