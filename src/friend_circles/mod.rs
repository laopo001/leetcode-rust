#![allow(unused)]
use crate::base::{index_of_vec, Solution};

impl Solution {
     pub fn find_circle_num(m: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let len: usize = m.len();

        let mut vec = (0..len).collect::<Vec<usize>>();
        for i in 0..len {
              // println!("{:?}", vec);
            if index_of_vec(&vec, i) != None {
                let mut arr: Vec<usize> = Vec::new();
                Solution::find_likes(&m, i, &mut arr);
                if arr.len() > 0 {
                    res = res + 1;
                }

                for x in arr {
                    let index = index_of_vec(&vec, x);
                    if let Some(i) = index {
                        vec.remove(i);
                    }
                }
            }
        }
        return res;
    }
    fn find_likes(m: &Vec<Vec<i32>>, x: usize, res: &mut Vec<usize>) {
        let len: usize = m.len();
        for i in 0..len {
            if m[x][i] == 1 {
                if index_of_vec(res, i) == None {
                    res.push(i);
                    Solution::find_likes(&m, i, res);
                }
            }
        }
    }
    fn find_likeds(m: &Vec<Vec<i32>>, x: usize) -> Vec<usize> {
        let mut res: Vec<usize> = Vec::new();
        for i in 0..m.len() {
            if m[i][x] == 1 {
                res.push(i)
            }
        }
        return res;
    }
}
