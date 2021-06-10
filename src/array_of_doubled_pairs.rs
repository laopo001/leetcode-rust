use std::collections::HashMap;

struct Solution;
impl Solution {
    // 52 ms, faster than 100.00% 
    pub fn can_reorder_doubled(arr: Vec<i32>) -> bool {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for x in arr.iter() {
            if let Some(v) = map.get_mut(x) {
                *v += 1;
            } else {
                map.insert(*x, 1);
            }
        }
        let mut left = vec![];
        let mut right = vec![];
        for x in arr.iter() {
            if (*x >= 0) {
                right.push(*x)
            } else {
                left.push(*x)
            }
        }
        left.sort();
        left.reverse();
        right.sort();
        left.append(&mut right);
        let array = left;
        // dbg!(&array, &map);
        for (i, x) in array.iter().enumerate() {
            if let Some(v) = map.get_mut(x) {
                if (*v > 0) {
                    *v -= 1;
                    if let Some(v2) = map.get_mut(&(x * 2)) {
                        if (*v2 > 0) {
                            *v2 -= 1;
                        } else {
                            return false;
                        }
                    } else {
                        return false;
                    }
                } else {
                    continue;
                }
            } else {
                return false;
            }
        }
        return true;
    }
}

#[test]
fn test() {
    assert!(!Solution::can_reorder_doubled(vec![
        -4, -6, -1, -2, -1, -1, -3, -8
    ]));
    assert!(Solution::can_reorder_doubled(vec![4, -2, 2, -4]));
}
