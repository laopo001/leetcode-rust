use crate::base::{Solution, JsArray};

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        // 0 ms, faster than 100.00%
        let mut arr: Vec<&str> = s.split(" ").collect();
        arr = arr.into_iter().filter(|x| { (*x).len() != 0 }).collect::<Vec<&str>>();
        match arr.last() {
            Some(s) => {
                return (*s).len() as i32;
            }
            None => {
                return 0;
            }
        }
    }
}