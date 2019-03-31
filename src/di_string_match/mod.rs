use crate::base::Solution;

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut min_v = 0;
        let mut max_v = s.len() as i32;
        let mut arr: Vec<i32> = vec![];
        let bytes = s.as_bytes();
        for i in 0..(s.len()) {
            if bytes[i] == b'D' {
                arr.push(max_v);
                max_v -= 1;
            } else {
                arr.push(min_v);
                min_v += 1;
            }
        }
        arr.push(min_v);

        return arr;
    }
    pub fn di_string_match2(s: String) -> Vec<i32> {
        let mut min_v = 0;
        let mut max_v = s.len() as i32;
        let mut arr: Vec<i32> = vec![0; s.len() + 1];
        let bytes = s.as_bytes();
        for i in 0..(s.len()) {
            if bytes[i] == b'D' {
                arr[i] = max_v;
                max_v -= 1;
            } else {
                arr[i] = min_v;
                min_v += 1;
            }
        }
        arr[s.len()] = min_v;
        return arr;
    }
}
