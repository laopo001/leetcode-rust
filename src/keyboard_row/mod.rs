fn index_of_vec<T: PartialEq>(v: &Vec<T>, i: T) -> Option<usize> {
    for item in 0..v.len() {
        if i == v[item] {
            return Some(item);
        }
    }
    return None;
}

pub fn find_words(words: Vec<String>) -> Vec<String> {
    let arr = vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'];
    let arr1 = vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'];
    let arr2 = vec!['z', 'x', 'c', 'v', 'b', 'n', 'm'];
    let mut res: Vec<String> = Vec::new();
    for item in words {
        let mut b = true;
        let lowers = item.to_ascii_lowercase();
        for c in lowers.chars() {
            if index_of_vec(&arr, c) == None {
                b = false;
                break;
            }
        }
        if (b) {
            res.push(item);
            continue;
        }
        b = true;
        for c in lowers.chars() {
            if index_of_vec(&arr1, c) == None {
                b = false;
                break;
            }
        }
        if (b) {
            res.push(item);
            continue;
        }
        b = true;
        for c in lowers.chars() {
            if index_of_vec(&arr2, c) == None {
                b = false;
                break;
            }
        }
        if (b) {
            res.push(item);
            continue;
        }
    }
    return res;
}