// https://leetcode.com/problems/find-lucky-integer-in-an-array/
// 3 ms, 2.2 MB

use std::collections::HashMap;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut counter: HashMap<i32, i32> = HashMap::new();
        for num in arr {
            let count = counter.entry(num).or_insert(0);
            *count+=1;
        }
        counter.retain(|k, v| k == v);
        let mut res = -1;
        for (k, v) in &counter {
            if *k > res { res = *k }
        }
        return res;
    }
}
