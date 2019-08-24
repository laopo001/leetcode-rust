use crate::base::Solution;

impl Solution {
	pub fn add_to_array_form(a: Vec<i32>, k: i32) -> Vec<i32> {
		let mut arr: Vec<i32> = vec![];
		let mut t = k;
		while t / 10 != 0 {
			arr.push(k % 10);
			t /= 10;
		}
		println!("{:?}", arr);
		vec![0]
	}
}