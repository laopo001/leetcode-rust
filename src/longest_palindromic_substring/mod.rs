use crate::base::Solution;

#[allow(dead_code)]
fn cmp(a: &str) -> bool {
	let len = a.len() / 2;
	for i in 0..(len) {
		if a.chars().nth(i) != a.chars().nth(a.len() - 1 - i) {
			return false;
		}
	}
	return true;
}

impl Solution {
	pub fn longest_palindrome(s: String) -> String {
		let len = s.len();
		let mut max_len = "";
		for i in 1..len {
			for j in 0..(i + 1) {
				if j > i / 2 {
					continue;
				}
				let z = s.get(j..(i + 1)).unwrap();
				if cmp(z) {
					if max_len.len() < z.len() {
						max_len = z;
					}
				}
			}
		}
		return max_len.to_string();
	}
	//
	// dp[i, j]   = 1                                 if i == j

	//           = s[] == s[j]                        if j = i + 1

	//           = s[i] == s[j] && dp[i + 1][j - 1]   if j > i + 1
	#[warn(deprecated)]
	pub fn longest_palindrome2(s: String) -> String {
		let mut dp: Vec<Vec<i32>> = vec![vec![0; s.len()]; s.len()];
		let c = s.as_bytes();
		let len = s.len();
		let mut max_len = 0;
		let mut max_index = 0;
		for i in 0..len {
			dp[i][i] = 1;
			max_len = 1;
			max_index = i;
		}
		for i in 0..len {
			let j = i + 1;
			if j < len && c[i] == c[j] {
				dp[i][j] = 1;
				max_len = 2;
				max_index = i;
			}
		}
		for i in 0..len {
			for j in 0..i {
				if i > j + 1 && c[i] == c[j] && dp[j + 1][i - 1] == 1 {
					dp[j][i] = 1;
					// println!("{:?},{:?}", j, i);
					if i - j + 1 > max_len {
						max_len = i - j + 1;
						max_index = j;
					}
				}
			}
		}
		unsafe {
			return s.get_unchecked(max_index..max_index + max_len).to_string();
		}
	}
}
