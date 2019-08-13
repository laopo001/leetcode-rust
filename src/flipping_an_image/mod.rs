#[allow(dead_code)]

fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // let coumnLen = a.len();
    // let rowLen = a[0].len();
    let mut b: Vec<Vec<i32>> = Vec::new();
    for row in a {
        let mut new_row: Vec<i32> = Vec::new();
        // row.reserve(row.len());
        for i in 0..row.len() {
            let curr_index = row.len() - 1 - i;
            let curr = if row[curr_index] == 0 { 1 } else { 0 };
            new_row.push(curr);
        }
        b.push(new_row);
    }
    return b;
}
