struct Solution;

fn run(i: i32, arr: &mut Vec<i32>) -> i32 {
    if i == 1 {
        return 1;
    };
    let res = run(i - 1, arr);
    arr.push(res);
    return i * res;
}

impl Solution {
    pub fn get_permutation(n: i32, mut k: i32) -> String {
        let mut arr: Vec<i32> = Vec::new();
        run(n, &mut arr);
        // dbg!(&arr);
        let mut selects: Vec<i32> = (1..=n).collect();
        let mut res = "".to_string();
        loop {
            if arr.len() == 0 {
                break;
            }
            let val = arr.pop().unwrap();
            let d = ((k - 1) / val) as usize;
            res += &selects[d].to_string();
            selects.splice(d..d + 1, Vec::<i32>::new().into_iter());
            k = k - (d as i32 * val);
            // dbg!(&selects, val, d, k, &res);
        }
        for item in selects {
            res += &item.to_string();
        }
        res
    }
}

#[test]
fn get_permutation() {
    assert_eq!(Solution::get_permutation(3, 3), "213".to_string());
    assert_eq!(Solution::get_permutation(4, 9), "2314".to_string());
}
