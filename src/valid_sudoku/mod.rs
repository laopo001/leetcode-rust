use crate::base::Solution;
use std::collections::HashSet;
// Runtime: 4 ms, faster than 94.00%
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let len = 9;
        let mut map = vec![false; 10];
        for y in 0..len {
            for x in 0..len {
                if board[y][x] == '.' {
                    continue;
                }
                if map[board[y][x].to_digit(10).unwrap() as usize - 1] {
                    return false;
                } else {
                    map[board[y][x].to_digit(10).unwrap() as usize - 1] = true;
                }
            }
            map.iter_mut().for_each(|x| {
                *x = false;
            });
            for x in 0..len {
                if board[x][y] == '.' {
                    continue;
                }
                if map[board[x][y].to_digit(10).unwrap() as usize - 1] {
                    return false;
                } else {
                    map[board[x][y].to_digit(10).unwrap() as usize - 1] = true;
                }
            }
            map.iter_mut().for_each(|x| {
                *x = false;
            });
        }
        let i = 3;
        for y in 0..i {
            for x in 0..i {
                for yy in 0..i {
                    for xx in 0..i {
                        if board[i * y + yy][i * x + xx] == '.' {
                            continue;
                        }
                        if map[board[i * y + yy][i * x + xx].to_digit(10).unwrap() as usize - 1] {
                            return false;
                        } else {
                            map[board[i * y + yy][i * x + xx].to_digit(10).unwrap() as usize - 1] =
                                true;
                        }
                    }
                }
                map.iter_mut().for_each(|x| {
                    *x = false;
                });
            }
        }
        true
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::is_valid_sudoku(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]),
        true
    );
}
