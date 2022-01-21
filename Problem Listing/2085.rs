// https://leetcode.com/problems/count-common-words-with-one-occurrence/
// 6 ms, 2.5 MB

use std::collections::HashMap;

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut mappings1: HashMap<String, i32> = HashMap::new();
        let mut mappings2: HashMap<String, i32> = HashMap::new();
        let mut res = 0;
        for x in words1 {
            let count = mappings1.entry(x).or_insert(0);
            *count+=1;
        }
        for y in words2 {
            let count = mappings2.entry(y).or_insert(0);
            *count+=1;
        }
        for (key, value) in mappings1 {
            let otherMap = match mappings2.get(&key) {
                Some(w) => *w,
                None => -1 as i32,
            };
            if value == 1 && otherMap == 1 as i32 { res+=1 }
        }
        return res;
    }
}
