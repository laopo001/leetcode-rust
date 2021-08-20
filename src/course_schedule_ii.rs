use std::usize;

struct Solution;

fn is_all_zero(arr: &Vec<i32>) -> bool {
    for i in arr {
        if *i != 0 {
            return false;
        }
    }
    return true;
}

fn run(
    start: i32,
    matrix: &Vec<Vec<i32>>,
    is_run: &mut Vec<i32>,
    stop: &mut bool,
) -> Vec<Vec<i32>> {
    let mut res = vec![];
    if (*stop) {
        return vec![];
    }
    if (is_run[start as usize] == 1) {
        *stop = true;
        return vec![];
    }

    is_run[start as usize] = 1;
    // dbg!(&is_run);
    for (i, arr) in matrix.iter().enumerate() {
        if (arr[start as usize] != 0) {
            let z = run(i as i32, matrix, is_run, stop);

            for mut i in z {
                let mut x = vec![start];
                x.append(&mut i);
                res.push(x);
            }
        }
    }
    is_run[start as usize] = 0;
    if (res.len() == 0) {
        res.push(vec![start]);
    }
    return res;
}

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut matrix = vec![vec![0; num_courses as usize]; num_courses as usize];
        for i in prerequisites.iter() {
            matrix[i[0] as usize][i[1] as usize] = 1;
        }
        let mut s = vec![];
        let mut is_run = vec![0; num_courses as usize];
        let mut stop = false;
        for (i, arr) in matrix.iter().enumerate() {
            if (is_all_zero(arr)) {
                let z = run(i as i32, &matrix, &mut is_run, &mut stop);
                for i in z {
                    s.push(i);
                }
            }
        }
        if (stop) {
            return vec![];
        }
        dbg!(&s);
        if (s.len() == 1) {
            return s[0].clone();
        } else {
            is_run = vec![0; num_courses as usize];
            let mut res = vec![];
            let mut i = 0;
            for x in s {
                if (i == 0) {
                    for y in x.iter() {
                        is_run[*y as usize] = 1;
                    }
                    res = x;
                    i += 1;
                    continue;
                }

                for i in (0..x.len()).rev() {
                    let y = x[i] as usize;
                    if (is_run[y] == 0) {
                        if (i == x.len() - 1) {
                            res.push(y as i32);
                        } else {
                            let start = x[i + 1];
                            let index = res.iter().position(|&r| r == start).unwrap();
                            
                            res.splice((index)..(index), [y as i32].iter().cloned());
                        }
                        is_run[y] = 1;
                    }
                }
                dbg!(&res);
                i += 1;
            }
            return res;
        }
        return vec![];
    }
}

#[test]
fn test() {
    // let mut arr = vec![
    //     vec![1, 0],
    //     vec![0, 3],
    //     vec![0, 2],
    //     vec![3, 2],
    //     vec![2, 5],
    //     vec![4, 5],
    //     vec![5, 6],
    //     vec![2, 4],
    // ];
    // let res = Solution::find_order(7, arr);
    let mut arr = vec![
        vec![5,8],vec![3,5],vec![1,9],vec![4,5],vec![0,2],vec![7,8],vec![4,9]
    ];
    let res = Solution::find_order(10, arr);


    dbg!(res);
}
