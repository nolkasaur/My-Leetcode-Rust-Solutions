// https://leetcode.com/problems/check-if-the-sentence-is-pangram/
// 0 ms, 2 MB

use std::collections::HashMap;

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut counter: HashMap<char, i32> = HashMap::new();
        for char in sentence.chars() {
            let count = counter.entry(char).or_insert(0);
            *count+=1;
        }
        return counter.len() == 26;
    }
}
