#[allow(dead_code)]
pub fn to_lower_case(str: String) -> String {
    // for i in 0..str.len() {
    //     let c = str.get(i..(i+1));
    // }
    let arr = str.as_bytes();
    let mut res = Vec::new();
    for item in arr {
        if *item <= (65 as u8) && *item <= (90 as u8) {
            res.push(item + 32);
        } else {
            res.push(*item);
        }
    }
    return String::from_utf8(res).unwrap();
}
