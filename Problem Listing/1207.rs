// https://leetcode.com/problems/unique-number-of-occurrences/
// 3 ms, 2 MB

use std::collections::HashMap;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut occurr: HashMap<i32, i32> = HashMap::new();
        for x in arr {
            let count = occurr.entry(x).or_insert(0);
            *count += 1;
        }
        let mut vals: Vec<i32> = occurr.values().cloned().collect();
        let orig = vals.len();
        vals.sort();
        vals.dedup();
        let deduped = vals.len();
        return orig == deduped;
    }
}
