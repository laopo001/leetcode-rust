struct Solution;

fn run(arr: &[u8], min_jump: usize, max_jump: usize, start: usize) -> bool {
    // let mut i = 0;
    let mut end = start + max_jump;
    let start = start + min_jump;

    if (start >= arr.len()) {
        return false;
    }
    if (end >= arr.len()) {
        end = arr.len() - 1;
    }

    let mut x = false;
    for i in start..=end {
        if (arr[i] == 0) {
            if (i == arr.len() - 1) {
                return true;
            }
            x = x || run(arr, min_jump, max_jump, i);
        }
    }
    return x;
}

impl Solution {
    // Time Limit Exceeded
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let arr: Vec<u8> = s.as_bytes().iter().map(|x| return *x - '0' as u8).collect();
        return run(arr.as_slice(), min_jump as usize, max_jump as usize, 0);
    }
}