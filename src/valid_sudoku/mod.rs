use crate::base::Solution;
use std::collections::HashSet;

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
        set.clear();
        for y in 0..i {
            for x in 0..i {
                if board[y][x] == '.' {
                    continue;
                }
                if set.contains(&board[y][x]) {
                    return false;
                } else {
                    set.insert(board[y][x]);
                }
            }
        }
        dbg!(1111);
        let mut all: Vec<HashSet<char>> = vec![set.clone()];
        let mut set_clone = set.clone();
        for x in i..len {
            for y in 0..i {
                if board[y][x - i] == '.' {
                    continue;
                }
                set_clone.remove(&board[y][x - i]);
            }
            for y in 0..i {
                if board[y][x] == '.' {
                    continue;
                }
                if set_clone.contains(&board[y][x]) {
                    return false;
                } else {
                    set_clone.insert(board[y][x]);
                }
            }
            all.push(set_clone.clone());
        }
        dbg!(2222);
        for y in i..len {
            for x in i..len {
                for z in 0..i {
                    if board[y - i][x - z] == '.' {
                        continue;
                    }
                    all[x - i].remove(&board[y - i][x - z]);
                }
                for z in 0..i {
                    if board[y][x - z] == '.' {
                        continue;
                    }
                    if all[x - i].contains(&board[y][x - z]) {
                        return false;
                    } else {
                        all[x - i].insert(board[y][x - z]);
                    }
                }
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
