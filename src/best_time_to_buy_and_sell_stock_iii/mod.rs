use crate::base::Solution;

impl Solution {
	pub fn max_profit_123(prices: Vec<i32>) -> i32 {
		if prices.len() == 0 { return 0; }
		use std::cmp::{max, min};
		let mut profit = 0;
		let len = prices.len();
		let mut valleys = vec![0; len];
		let mut peaks = vec![0; len];
		let mut valley = prices[0];
		for i in 1..len {
			valley = min(valley, prices[i]);
			valleys[i] = max(valleys[i - 1], prices[i] - valley);
		}
//		println!("{:?}", valleys);
		let mut peak = prices[len - 1];
		for i in (0..(len - 1)).rev() {
			peak = max(peak, prices[i]);
			peaks[i] = max(peaks[i + 1], peak - prices[i]);
		}
//		println!("{:?}", peaks);
		for i in 0..len {
			profit = max(profit, valleys[i] + peaks[i]);
		}
		return profit;
	}
}
