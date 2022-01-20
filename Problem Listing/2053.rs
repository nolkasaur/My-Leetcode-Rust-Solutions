// https://leetcode.com/problems/kth-distinct-string-in-an-array/
// 12 ms, 2.4 MB

use std::collections::HashMap;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut mappings: HashMap<String, i32> = HashMap::new();
        for x in &arr {
            let count = mappings.entry(x.to_string()).or_insert(0);
            *count+=1;
        }
        let mut count = 0;
        for y in &arr {
            if *mappings.get(&y.to_string()).unwrap() == 1 as i32 {
                count+=1;
                if count == k { return y.to_string(); }
            }
        }
        return String::from("");
    }
}
