// https://leetcode.com/problems/check-if-all-characters-have-equal-number-of-occurrences/
// 1 ms, 2.3 MB

use std::collections::HashMap;

impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut mappings: HashMap<char, i32> = HashMap::new();
        for x in s.chars() {
            let count = mappings.entry(x).or_insert(0);
            *count+=1;
        }
        let count = *mappings.get(&s.chars().nth(0).unwrap()).unwrap();
        for (key, value) in mappings {
            if value != count { return false }
        }
        return true;
    }
}
