struct Solution;

impl Solution {
	//  0 ms, faster than 100.00%
	pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
		let mut s: Vec<usize> = vec![];
		let mut index: Option<usize> = None;
		let mut left = 0;
		let mut right = 0;
		for i in 0..seats.len() {
			if seats[i] == 1 {
				if index.is_none() {
					left = i;
				}
				if index.is_some() {
					s.push(i - index.unwrap());
				}
				index = Some(i);
			}
			if seats.len() - 1 == i && index.is_some() {
				right =  i - index.unwrap();
			}
		}
		// println!("{:?},{:?},{:?}", s,left,right);
		if s.len() == 0 {
			return std::cmp::max(left,right) as i32;
		}
		let mut max = std::i32::MIN;
		for i in 0..s.len() {
			max = std::cmp::max(max, s[i] as i32 / 2);
		}
		max = std::cmp::max(max, left as i32);
		max = std::cmp::max(max, right as i32);
		max
	}
}