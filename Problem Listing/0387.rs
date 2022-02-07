// https://leetcode.com/problems/first-unique-character-in-a-string/submissions/
// 24 ms, 2.2 MB

use std::collections::HashMap;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut counter: HashMap<char, u32> = HashMap::new();
        for char in s.chars() {
            let count = counter.entry(char).or_insert(0);
            *count += 1;
        }
        let mut index = 0;
        for char in s.chars() {
            if *counter.get(&char).unwrap() == 1 {
                return index;
            }
            index += 1;
        }
        return -1;
    }
}
