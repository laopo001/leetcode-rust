#![allow(unconditional_recursion)]
struct Solution;

fn quicksort(a: &mut Vec<i32>, l: usize, r: usize) {
    // dbg!(&a, l, r);
    if (l >= r) {
        return;
    }
    let mut index = (l + r) / 2;
    let pivot = a[index];
    let mut i = l;
    let mut j = r;
    while (i < j) {
        while (a[i] < pivot) {
            i += 1;
        }
        while (a[j] > pivot) {
            // dbg!("----------------");
            j -= 1;
        }
        if i < index && j > index {
            swap(a, i, j);
            i += 1;
            j -= 1;
        } else if i == index {
            swap(a, i, j);
            index = j;
            i += 1;
        } else if j == index {
            swap(a, i, j);
            index = i;
            j -= 1;
        }
    }
    if index >= 1 {
        quicksort(a, l, index - 1);
    }

    quicksort(a, index + 1, r);
}

fn swap(a: &mut Vec<i32>, i: usize, j: usize) {
    let tmp = a[i];
    a[i] = a[j];
    a[j] = tmp;
}
// Runtime: 8 ms, faster than 92.86%
impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        quicksort(&mut nums, 0, len - 1);
        nums
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::sort_array(vec![5, 4, 3, 2, 1]),
        vec![1, 2, 3, 4, 5]
    );
    assert_eq!(
        Solution::sort_array(vec![-85, -90, -87, -74, -71]),
        vec![-90, -87, -85, -74, -71]
    );
    assert_eq!(Solution::sort_array(vec![-2, 3, -5]), vec![-5, -2, 3]);
    assert_eq!(Solution::sort_array(vec![-1, 1]), vec![-1, 1]);
    assert_eq!(
        Solution::sort_array(vec![-1, 5, 1, 1, 1]),
        vec![-1, 1, 1, 1, 5]
    );
    assert_eq!(Solution::sort_array(vec![-1, -10, -8]), vec![-10, -8, -1]);

    assert_eq!(Solution::sort_array(vec![0, 0, 1]), vec![0, 0, 1]);
    assert_eq!(
        Solution::sort_array(vec![5, 1, 1, 2, 0, 0]),
        vec![0, 0, 1, 1, 2, 5]
    );
    assert_eq!(Solution::sort_array(vec![5, 2, 3, 1]), vec![1, 2, 3, 5]);
}
