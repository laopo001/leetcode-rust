struct Solution;
// Runtime: 0 ms, faster than 100.00%
impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut res = vec![];
        let mut temp = vec![];
        let mut map = arr1.iter().fold(HashMap::<i32, usize>::new(), |mut a, b| {
            if let Some(x) = a.get_mut(&b) {
                *x += 1;
            } else {
                a.insert(*b, 1);
                temp.push(*b);
            }
            a
        });
        arr2.iter().for_each(|x| {
            for i in 0..(*map.get(&x).unwrap()) {
                res.push(*x);
                *map.get_mut(x).unwrap() = 0;
            }
        });
        let mut temp = arr1
            .into_iter()
            .filter(|x| *map.get(x).unwrap() != 0)
            .collect::<Vec<i32>>();
        temp.sort();
        temp.into_iter().for_each(|x| {
            res.push(x);
        });
        res
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::relative_sort_array(vec![28, 6, 22, 8, 44, 17], vec![22, 28, 8, 6]),
        vec![22, 28, 8, 6, 17, 44]
    )
}
