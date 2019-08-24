use crate::base::Solution;

impl Solution {
	pub fn add_to_array_form(a: Vec<i32>, k: i32) -> Vec<i32> {
		let mut b: Vec<i32> = vec![];
		let mut t = k;
		while t / 10 != 0 {
			b.push(t % 10);
			t /= 10;
		}
		b.push(t);
//		println!("{:?}", arr);
		let mut a = a;
		a.reverse();
		let len = if a.len() > b.len() { a.len() } else { b.len() };
		for i in 0..len{
			
		}
		vec![0]
	}
}