struct Solution;

impl Solution {
	// 0 ms, faster than 100.00%
	pub fn reverse_vowels(s: String) -> String {
		let mut vowels: Vec<u8> = vec![];
		let mut arr: Vec<usize> = vec![];
		for i in 0..s.len() {
			let item = s.get(i..i + 1).unwrap();
			if item == "a" || item == "e" || item == "i" || item == "o" || item == "u"
				|| item == "A" || item == "E" || item == "I" || item == "O" || item == "U"
			{
				vowels.push(item.as_bytes()[0]);
				arr.push(i);
			}
		}
		vowels.reverse();
		let mut s = s;
		unsafe {
			let mut res = s.as_bytes_mut();
			for i in 0..arr.len() {
				let index = arr[i];
				let v = vowels[i];
				res[index] = v;
			}
			return String::from_utf8(res.to_vec()).unwrap();
		}
	}
}