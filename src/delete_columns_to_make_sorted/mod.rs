struct Solution;

impl Solution {
    pub fn min_deletion_size(a: Vec<String>) -> i32 {
        let cout = a.len();
        if cout == 0 {
            return 0;
        }
        let len = a[0].len();
        // let z = "".to_string();
        // let b: Vec<_> = z.bytes().collect();
        // let bytes: Vec<_> = a.map(|x| *x.as_bytes());
        let mut res = 0;
        for i in 0..len {
            for j in 0..(cout - 1) {
                if a[j].as_bytes()[i] > a[j + 1].as_bytes()[i] {
                    res += 1;
                    break;
                }
            }
        }
        return res;
    }
}
