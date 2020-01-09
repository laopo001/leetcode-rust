struct Solution;

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut map = [0; 26];
        let a_num = 'a' as u8;
        for c in chars.bytes() {
            map[(c - a_num) as usize] += 1;
        }
        let mut count = 0;
        'l2: for word in words {
            let mut map_copy = map;
            for c in word.bytes() {
                map_copy[(c - a_num) as usize] -= 1;
                if map_copy[(c - a_num) as usize] < 0 {
                    continue 'l2;
                }
            }
            count += word.len();
        }
        return count as i32;
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::count_characters(
            vec![
                "hello".to_string(),
                "world".to_string(),
                "leetcode".to_string()
            ],
            "welldonehoneyr".to_string()
        ),
        10
    );
    assert_eq!(
        Solution::count_characters(
            vec![
                "cat".to_string(),
                "bt".to_string(),
                "hat".to_string(),
                "tree".to_string()
            ],
            "atach".to_string()
        ),
        6
    );
}
