use crate::base::Solution;
use std::collections::HashSet;
// Runtime: 4 ms, faster than 94.00%
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let len = 9;
        let mut set = HashSet::<char>::new();
        for y in 0..len {
            let mut set = HashSet::<char>::new();
            for x in 0..len {
                if board[y][x] == '.' {
                    continue;
                }
                if set.contains(&board[y][x]) {
                    return false;
                } else {
                    set.insert(board[y][x]);
                }
            }
            set.clear();
            for x in 0..len {
                if board[x][y] == '.' {
                    continue;
                }
                if set.contains(&board[x][y]) {
                    return false;
                } else {
                    set.insert(board[x][y]);
                }
            }
            set.clear();
        }
        let i = 3;
        for y in 0..i {
            for x in 0..i {
                for yy in 0..i {
                    for xx in 0..i {
                        if board[i * y + yy][i * x + xx] == '.' {
                            continue;
                        }
                        if set.contains(&board[i * y + yy][i * x + xx]) {
                            return false;
                        } else {
                            set.insert(board[i * y + yy][i * x + xx]);
                        }
                    }
                }
                // dbg!(i * y, i * x, &set);
                set.clear();
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
