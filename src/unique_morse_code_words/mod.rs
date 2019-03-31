use crate::base::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let arr = [
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
            "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
            "-.--", "--..",
        ];
        let mut set: HashSet<String> = HashSet::new();
        for s in words {
            let mut temp = "".to_string();
            for b in s.as_bytes() {
                let index = *b - b'a';
                temp += arr[index as usize];
            }
            set.insert(temp);
        }
        return set.len() as i32;
    }
}
