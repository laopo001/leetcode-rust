struct Solution;

fn run(arr: &Vec<i32>, start: i32, map: &mut Vec<i32>) -> bool {
    if (map[start as usize] == 1) {
        return false;
    }
    map[start as usize] = 1;
    if (arr[start as usize] == 0) {
        return true;
    }
    let mut i = 0;
    let mut a = false;
    if (start - arr[start as usize] >= 0) {
        a = run(arr, start - arr[start as usize], map);
    }
    let mut b = false;
    if (start + arr[start as usize] < arr.len() as i32) {
        b = run(arr, start + arr[start as usize], map);
    }
    return a || b;
}

impl Solution {
    // 4 ms, faster than 92.59%
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        return run(&arr, start, &mut vec![0; arr.len()]);
    }
}
