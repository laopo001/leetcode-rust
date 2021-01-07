struct Solution;

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut res = 0_f64;
        let mut max = 0_f64;
        let mut min = std::f64::INFINITY;
        for i in salary.iter() {
            if (min > *i as f64) {
                min = *i as f64;
            }
            if (max < *i as f64) {
                max = *i as f64;
            }
            res += *i as f64;
        }

        (res - min - max) / ((salary.len() - 2) as f64)
    }
}
