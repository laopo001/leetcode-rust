impl Solution {
	pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
		let arr = vec![0; coins.len()];
		let mut res = 0;
		let mut it = 0;
		while it < coins.len() {
			if res < amount {
				it = std::cmp::max(0, it - 1);
				for i in (0..arr.len()).rev() {
					res += arr[i] * coins[i];
					if it == i {
						arr[i] += 1;
					}
				}
			} else {
				it += 1;
				arr[it] += 1;
			}
		}
		-1
	}
}