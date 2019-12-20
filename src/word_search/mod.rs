struct Solution;

fn check(x: usize, y: usize, len_x: usize, len_y: usize, map: &mut Vec<Vec<bool>>) -> bool {
    if (x >= 0 && x < len_x && y >= 0 && y < len_y && map[x][y]) {
        return true;
    } else {
        return false;
    }
}

fn run(
    x: usize,
    y: usize,
    index: usize,
    word: &Vec<char>,
    map: &mut Vec<Vec<bool>>,
    board: &Vec<Vec<char>>,
) -> bool {
    let len_x = board.len();
    let len_y = board[0].len();
    // println!("{},{},{},{},{}", x, y, index, board[x][y], word[index]);
    // println!("::::{:?}", map);
    let mut b = board[x][y] == word[index];
    if !b {
        return false;
    }
    if (index == word.len() - 1 && b) {
        return true;
    }
    if check(x - 1, y, len_x, len_y, map) {
        map[x][y] = false;
        if b && run(x - 1, y, index + 1, word, map, board) {
            return true;
        }
        map[x][y] = true;
    }
    if check(x + 1, y, len_x, len_y, map) {
        map[x][y] = false;
        if b && run(x + 1, y, index + 1, word, map, board) {
            return true;
        }
        map[x][y] = true;
    }
    if check(x, y - 1, len_x, len_y, map) {
        map[x][y] = false;
        if b && run(x, y - 1, index + 1, word, map, board) {
            return true;
        }
        map[x][y] = true;
    }
    if check(x, y + 1, len_x, len_y, map) {
        // println!("::::{},{},{},{},{}", x, y+1,index+1,board[x][y+1],word[index+1]);
        map[x][y] = false;
        if b && run(x, y + 1, index + 1, word, map, board) {
            return true;
        }
        map[x][y] = true;
    }
    // println!("::::{:?}", 123);
    return false;
}
// Runtime: 24 ms, faster than 26.92%
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        if word.len() == 0 {
            return false;
        }
        let mut map: Vec<Vec<bool>> = board
            .iter()
            .map(|x| x.iter().map(|y| true).collect())
            .collect();
        let word_arr: Vec<char> = word.chars().collect();
        // run(0, 0, 0, &word_arr, &mut map, &board);
        for i in 0..board.len() {
            let val = &board[i];
            for j in 0..val.len() {
                if (run(i, j, 0, &word_arr, &mut map, &board)) {
                    return true;
                } else {
                    map.iter_mut()
                        .for_each(|x| x.iter_mut().for_each(|y| *y = true));
                }
            }
        }
        false
    }
}
